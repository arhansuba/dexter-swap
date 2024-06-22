// SPDX-License-Identifier: BUSL-1.1

mod full_math;
mod sqrt_price_math;

use full_math::*;
use sqrt_price_math::*;

/// Computes the result of swapping some amount in, or amount out, given the parameters of the swap
/// The fee, plus the amount in, will never exceed the amount remaining if the swap's `amount_specified` is positive
/// Returns the price after swapping the amount in/out, not to exceed the price target
fn compute_swap_step(
    sqrt_ratio_current_x96: u160,
    sqrt_ratio_target_x96: u160,
    liquidity: u128,
    amount_remaining: i256,
    fee_pips: u24,
) -> (u160, u256, u256, u256) {
    let zero_for_one = sqrt_ratio_current_x96 >= sqrt_ratio_target_x96;
    let exact_in = amount_remaining >= 0;

    let (sqrt_ratio_next_x96, mut amount_in, mut amount_out, fee_amount);

    if exact_in {
        let amount_remaining_less_fee = full_mul_div(
            amount_remaining.abs().into(),
            1_000_000 - fee_pips.into(),
            1_000_000.into(),
        );

        amount_in = if zero_for_one {
            get_amount0_delta(
                sqrt_ratio_target_x96,
                sqrt_ratio_current_x96,
                liquidity,
                true,
            )
        } else {
            get_amount1_delta(
                sqrt_ratio_current_x96,
                sqrt_ratio_target_x96,
                liquidity,
                true,
            )
        };

        if amount_remaining_less_fee >= amount_in {
            sqrt_ratio_next_x96 = sqrt_ratio_target_x96;
        } else {
            sqrt_ratio_next_x96 = if zero_for_one {
                get_next_sqrt_price_from_input(
                    sqrt_ratio_current_x96,
                    liquidity,
                    amount_remaining_less_fee,
                    true,
                )
            } else {
                get_next_sqrt_price_from_output(
                    sqrt_ratio_current_x96,
                    liquidity,
                    amount_remaining_less_fee,
                    true,
                )
            };
        }
    } else {
        amount_out = if zero_for_one {
            get_amount1_delta(
                sqrt_ratio_target_x96,
                sqrt_ratio_current_x96,
                liquidity,
                false,
            )
        } else {
            get_amount0_delta(
                sqrt_ratio_current_x96,
                sqrt_ratio_target_x96,
                liquidity,
                false,
            )
        };

        if amount_remaining.abs() as u256 >= amount_out {
            sqrt_ratio_next_x96 = sqrt_ratio_target_x96;
        } else {
            sqrt_ratio_next_x96 = if zero_for_one {
                get_next_sqrt_price_from_output(
                    sqrt_ratio_current_x96,
                    liquidity,
                    amount_remaining.abs() as u256,
                    true,
                )
            } else {
                get_next_sqrt_price_from_input(
                    sqrt_ratio_current_x96,
                    liquidity,
                    amount_remaining.abs() as u256,
                    true,
                )
            };
        }
    }

    let max = sqrt_ratio_target_x96 == sqrt_ratio_next_x96;

    // Calculate input/output amounts
    if zero_for_one {
        amount_in = if max && exact_in {
            amount_in
        } else {
            get_amount0_delta(
                sqrt_ratio_next_x96,
                sqrt_ratio_current_x96,
                liquidity,
                true,
            )
        };
        amount_out = if max && !exact_in {
            amount_out
        } else {
            get_amount1_delta(
                sqrt_ratio_next_x96,
                sqrt_ratio_current_x96,
                liquidity,
                false,
            )
        };
    } else {
        amount_in = if max && exact_in {
            amount_in
        } else {
            get_amount1_delta(
                sqrt_ratio_current_x96,
                sqrt_ratio_next_x96,
                liquidity,
                true,
            )
        };
        amount_out = if max && !exact_in {
            amount_out
        } else {
            get_amount0_delta(
                sqrt_ratio_current_x96,
                sqrt_ratio_next_x96,
                liquidity,
                false,
            )
        };
    }

    // Cap the output amount to not exceed the remaining output amount
    if !exact_in && amount_out > amount_remaining.abs() as u256 {
        amount_out = amount_remaining.abs() as u256;
    }

    if exact_in && sqrt_ratio_next_x96 != sqrt_ratio_target_x96 {
        // Didn't reach the target, so take the remainder of the maximum input as fee
        fee_amount = amount_remaining.abs() as u256 - amount_in;
    } else {
        fee_amount = full_mul_div_rounding_up(
            amount_in,
            fee_pips.into(),
            1_000_000 - fee_pips.into(),
        );
    }

    (sqrt_ratio_next_x96, amount_in, amount_out, fee_amount)
}

fn main() {
    // Example usage
    let sqrt_ratio_current_x96: u160 = 12345;
    let sqrt_ratio_target_x96: u160 = 23456;
    let liquidity: u128 = 100000;
    let amount_remaining: i256 = 1000;
    let fee_pips: u24 = 50;

    let (sqrt_ratio_next_x96, amount_in, amount_out, fee_amount) = compute_swap_step(
        sqrt_ratio_current_x96,
        sqrt_ratio_target_x96,
        liquidity,
        amount_remaining,
        fee_pips,
    );

    println!("Next sqrt ratio: {}", sqrt_ratio_next_x96);
    println!("Amount in: {}", amount_in);
    println!("Amount out: {}", amount_out);
    println!("Fee amount: {}", fee_amount);
}
