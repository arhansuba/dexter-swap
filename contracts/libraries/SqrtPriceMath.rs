// SPDX-License-Identifier: BUSL-1.1

mod low_gas_safe_math;
mod safe_cast;
mod full_math;
mod unsafe_math;
mod fixed_point_96;

use low_gas_safe_math::*;
use safe_cast::*;
use full_math::*;
use unsafe_math::*;
use fixed_point_96::*;

/// Gets the next sqrt price given a delta of token0
/// Always rounds up, because in the exact output case (increasing price) we need to move the price at least
/// far enough to get the desired output amount, and in the exact input case (decreasing price) we need to move the
/// price less in order to not send too much output.
/// The most precise formula for this is liquidity * sqrtPX96 / (liquidity +- amount * sqrtPX96),
/// if this is impossible because of overflow, we calculate liquidity / (liquidity / sqrtPX96 +- amount).
/// Returns the price after adding or removing amount, depending on add
fn get_next_sqrt_price_from_amount0_rounding_up(
    sqrt_px96: u160,
    liquidity: u128,
    amount: u256,
    add: bool,
) -> u160 {
    // Short circuit amount == 0 because the result is otherwise not guaranteed to equal the input price
    if amount == 0 {
        return sqrt_px96;
    }

    let numerator1 = u256::from(liquidity) << FixedPoint96::RESOLUTION;

    if add {
        if let Some(product) = amount.checked_mul(u256::from(sqrt_px96)) {
            let denominator = numerator1.checked_add(product).expect("Denominator overflow");
            if denominator >= numerator1 {
                // Always fits in 160 bits
                return full_mul_div_rounding_up(numerator1, u256::from(sqrt_px96), denominator).to_u160();
            }
        }
        return unsafe_div_rounding_up(numerator1, unsafe_div(numerator1, u256::from(sqrt_px96)).checked_add(amount).expect("Denominator overflow"))
    } else {
        let product = amount.checked_mul(u256::from(sqrt_px96)).expect("Product overflow");
        // If the product overflows, we know the denominator underflows
        // In addition, we must check that the denominator does not underflow
        assert!(product <= u256::MAX - numerator1, "Product overflow");
        let denominator = numerator1.checked_sub(product).expect("Denominator underflow");
        return full_mul_div_rounding_up(numerator1, u256::from(sqrt_px96), denominator).to_u160();
    }
}

/// Gets the next sqrt price given a delta of token1
/// Always rounds down, because in the exact output case (decreasing price) we need to move the price at least
/// far enough to get the desired output amount, and in the exact input case (increasing price) we need to move the
/// price less in order to not send too much output.
/// The formula we compute is within <1 wei of the lossless version: sqrtPX96 +- amount / liquidity
/// Returns the price after adding or removing amount
fn get_next_sqrt_price_from_amount1_rounding_down(
    sqrt_px96: u160,
    liquidity: u128,
    amount: u256,
    add: bool,
) -> u160 {
    // If we're adding (subtracting), rounding down requires rounding the quotient down (up)
    // In both cases, avoid a mulDiv for most inputs
    if add {
        let quotient = if amount <= u160::MAX.into() {
            (amount << FixedPoint96::RESOLUTION) / liquidity
        } else {
            full_mul_div(amount, FixedPoint96::Q96, liquidity)
        };
        return u256::from(sqrt_px96).checked_add(quotient).expect("Price overflow").to_u160();
    } else {
        let quotient = if amount <= u160::MAX.into() {
            unsafe_div_rounding_up(amount << FixedPoint96::RESOLUTION, liquidity)
        } else {
            full_mul_div_rounding_up(amount, FixedPoint96::Q96, liquidity)
        };
        assert!(sqrt_px96 > quotient, "Price underflow");
        // Always fits 160 bits
        return sqrt_px96.checked_sub(quotient).expect("Price underflow");
    }
}

/// Gets the next sqrt price given an input amount of token0 or token1
/// Throws if price or liquidity are 0, or if the next price is out of bounds
/// Returns the price after adding the input amount to token0 or token1
fn get_next_sqrt_price_from_input(
    sqrt_px96: u160,
    liquidity: u128,
    amount_in: u256,
    zero_for_one: bool,
) -> u160 {
    assert!(sqrt_px96 > 0, "Price must be greater than 0");
    assert!(liquidity > 0, "Liquidity must be greater than 0");

    // Round to make sure that we don't pass the target price
    if zero_for_one {
        get_next_sqrt_price_from_amount0_rounding_up(sqrt_px96, liquidity, amount_in, true)
    } else {
        get_next_sqrt_price_from_amount1_rounding_down(sqrt_px96, liquidity, amount_in, true)
    }
}

/// Gets the next sqrt price given an output amount of token0 or token1
/// Throws if price or liquidity are 0 or the next price is out of bounds
/// Returns the price after removing the output amount of token0 or token1
fn get_next_sqrt_price_from_output(
    sqrt_px96: u160,
    liquidity: u128,
    amount_out: u256,
    zero_for_one: bool,
) -> u160 {
    assert!(sqrt_px96 > 0, "Price must be greater than 0");
    assert!(liquidity > 0, "Liquidity must be greater than 0");

    // Round to make sure that we pass the target price
    if zero_for_one {
        get_next_sqrt_price_from_amount1_rounding_down(sqrt_px96, liquidity, amount_out, false)
    } else {
        get_next_sqrt_price_from_amount0_rounding_up(sqrt_px96, liquidity, amount_out, false)
    }
}

/// Gets the amount0 delta between two prices
/// Calculates liquidity / sqrt(lower) - liquidity / sqrt(upper),
/// i.e., liquidity * (sqrt(upper) - sqrt(lower)) / (sqrt(upper) * sqrt(lower))
/// Returns amount0 required to cover a position of size liquidity between the two passed prices
fn get_amount0_delta(
    sqrt_ratio_a_x96: u160,
    sqrt_ratio_b_x96: u160,
    liquidity: u128,
    round_up: bool,
) -> u256 {
    let (mut sqrt_ratio_a_x96, mut sqrt_ratio_b_x96) = if sqrt_ratio_a_x96 > sqrt_ratio_b_x96 {
        (sqrt_ratio_b_x96, sqrt_ratio_a_x96)
    } else {
        (sqrt_ratio_a_x96, sqrt_ratio_b_x96)
    };

    assert!(sqrt_ratio_a_x96 > 0, "Price must be greater than 0");

    let numerator1 = u256::from(liquidity) << FixedPoint96::RESOLUTION;
    let numerator2 = sqrt_ratio_b_x96.checked_sub(sqrt_ratio_a_x96).expect("Underflow");

    if round_up {
        unsafe_div_rounding_up(
            full_mul_div_rounding_up(numerator1, numerator2, sqrt_ratio_b_x96),
            sqrt_ratio_a_x96,
        )
    } else {
        full_mul_div(numerator1, numerator2, sqrt_ratio_b_x96).checked_div(sqrt_ratio_a_x96).expect("Overflow")
    }
}

/// Gets the amount1 delta between two prices
/// Calculates liquidity * (sqrt(upper) - sqrt(lower))
/// Returns amount1 required to cover a position of size liquidity between the two passed prices
fn get_amount1_delta(
    sqrt_ratio_a_x96: u160,
    sqrt_ratio_b_x96: u160,
    liquidity: u128,
    round_up: bool,
) -> u256 {
    let (mut sqrt_ratio_a_x96, mut sqrt_ratio_b_x96) = if sqrt_ratio_a_x96 > sqrt_ratio_b_x96 {
        (sqrt_ratio_b_x96, sqrt_ratio_a_x96)
    } else {
        (sqrt_ratio_a_x96, sqrt_ratio_b_x96)
    };

    let numerator = sqrt_ratio_b_x96.checked_sub(sqrt_ratio_a_x96).expect("Underflow");

    if round_up {
        full_mul_div_rounding_up(liquidity, numerator, FixedPoint96::Q96)
    } else {
        full_mul_div(liquidity, numerator, FixedPoint96::Q96)
    }
}

/// Helper that gets signed token0 delta
/// Returns amount0 corresponding to the passed liquidity_delta between the two prices
fn get_amount0_delta_signed(
    sqrt_ratio_a_x96: u160,
    sqrt_ratio_b_x96: u160,
    liquidity_delta: i128,
) -> i256 {
    if liquidity_delta < 0 {
        -get_amount0_delta(sqrt_ratio_a_x96, sqrt_ratio_b_x96, -liquidity_delta).to_i256()
    } else {
        get_amount0_delta(sqrt_ratio_a_x96, sqrt_ratio_b_x96, liquidity_delta).to_i256()
    }
}

/// Helper that gets signed token1 delta
/// Returns amount1 corresponding to the passed liquidity_delta between the two prices
fn get_amount1_delta_signed(
    sqrt_ratio_a_x96: u160,
    sqrt_ratio_b_x96: u160,
    liquidity_delta: i128,
) -> i256 {
    if liquidity_delta < 0 {
        -get_amount1_delta(sqrt_ratio_a_x96, sqrt_ratio_b_x96, -liquidity_delta).to_i256()
    } else {
        get_amount1_delta(sqrt_ratio_a_x96, sqrt_ratio_b_x96, liquidity_delta).to_i256()
    }
}

fn main() {
    // Example usage
    let sqrt_px96: u160 = 12345;
    let liquidity: u128 = 100000;
    let amount_in: u256 = 1000;
    let zero_for_one: bool = true;

    let next_sqrt_price = get_next_sqrt_price_from_input(sqrt_px96, liquidity, amount_in, zero_for_one);
    println!("Next sqrt price from input: {}", next_sqrt_price);
}
