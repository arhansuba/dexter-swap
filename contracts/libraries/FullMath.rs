/// SPDX-License-Identifier: MIT
/// A library for handling 512-bit math functions
/// Facilitates multiplication and division that can have overflow of an intermediate value without any loss of precision
/// Credit to Remco Bloemen under MIT license https://xn--2-umb.com/21/muldiv

pub struct FullMath;

impl FullMath {
    /// Calculates floor(a×b÷denominator) with full precision. Panics if result overflows a u256 or denominator == 0
    pub fn mul_div(a: u256, b: u256, denominator: u256) -> u256 {
        // 512-bit multiply [prod1 prod0] = a * b
        // Compute the product mod 2**256 and mod 2**256 - 1
        // then use the Chinese Remainder Theorem to reconstruct
        // the 512 bit result. The result is stored in two 256
        // variables such that product = prod1 * 2**256 + prod0
        let mut prod0: u256 = a * b;
        let mut prod1: u256;
        
        // Calculate mod 2**256
        let mm = a.wrapping_mul(b);
        prod1 = mm.wrapping_sub(prod0);
        if mm < prod0 {
            prod1 = prod1.wrapping_sub(1);
        }

        // Handle non-overflow cases, 256 by 256 division
        if prod1 == 0 {
            assert!(denominator > 0, "Division by zero");
            return prod0 / denominator;
        }

        // Make sure the result is less than 2**256
        assert!(denominator > prod1, "Division by zero or overflow");

        ///////////////////////////////////////////////
        // 512 by 256 division.
        ///////////////////////////////////////////////

        // Compute remainder using wrapping_mul
        let remainder = a.wrapping_mul(b) % denominator;

        // Subtract 256 bit number from 512 bit number
        prod1 = prod1.wrapping_sub(if remainder > prod0 { 1 } else { 0 });
        prod0 = prod0.wrapping_sub(remainder);

        // Factor powers of two out of denominator
        let twos = !denominator & denominator;
        // Divide denominator by power of two
        let denominator = denominator / twos;

        // Divide [prod1 prod0] by the factors of two
        prod0 = prod0 / twos;

        // Shift in bits from prod1 into prod0
        let twos = !twos;
        prod0 |= prod1.wrapping_mul(twos);

        // Invert denominator mod 2**256
        let mut inv = (3 * denominator) ^ 2;
        inv *= 2 - denominator * inv;
        inv *= 2 - denominator * inv;
        inv *= 2 - denominator * inv;
        inv *= 2 - denominator * inv;
        inv *= 2 - denominator * inv;

        // Because the division is now exact we can divide by multiplying
        // with the modular inverse of denominator. This will give us the
        // correct result modulo 2**256. Since the preconditions guarantee
        // that the outcome is less than 2**256, this is the final result.
        prod0 * inv
    }

    /// Calculates ceil(a×b÷denominator) with full precision. Panics if result overflows a u256 or denominator == 0
    pub fn mul_div_rounding_up(a: u256, b: u256, denominator: u256) -> u256 {
        let result = FullMath::mul_div(a, b, denominator);
        if a.wrapping_mul(b) % denominator > 0 {
            assert!(result < u256::max_value(), "Overflow");
            return result + 1;
        }
        result
    }
}

/// Define u256 type representing a 256-bit unsigned integer
type u256 = [u64; 4];
