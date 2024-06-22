// SPDX-License-Identifier: GPL-2.0-or-later

/// Optimized overflow and underflow safe math operations
pub mod low_gas_safe_math {
    /// Returns x + y, panics if sum overflows u128
    ///
    /// # Arguments
    ///
    /// * `x` - The augend
    /// * `y` - The addend
    ///
    /// # Returns
    ///
    /// * `z` - The sum of x and y
    ///
    /// # Panics
    ///
    /// Panics if overflow occurs during the addition.
    pub fn add(x: u128, y: u128) -> u128 {
        let z = x.checked_add(y).expect("Addition overflow");
        z
    }

    /// Returns x - y, panics if underflows
    ///
    /// # Arguments
    ///
    /// * `x` - The minuend
    /// * `y` - The subtrahend
    ///
    /// # Returns
    ///
    /// * `z` - The difference of x and y
    ///
    /// # Panics
    ///
    /// Panics if underflow occurs during the subtraction.
    pub fn sub(x: u128, y: u128) -> u128 {
        let z = x.checked_sub(y).expect("Subtraction underflow");
        z
    }

    /// Returns x * y, panics if overflows u128
    ///
    /// # Arguments
    ///
    /// * `x` - The multiplicand
    /// * `y` - The multiplier
    ///
    /// # Returns
    ///
    /// * `z` - The product of x and y
    ///
    /// # Panics
    ///
    /// Panics if overflow occurs during the multiplication.
    pub fn mul(x: u128, y: u128) -> u128 {
        let z = x.checked_mul(y).expect("Multiplication overflow");
        z
    }

    /// Returns x + y, panics if overflows or underflows i128
    ///
    /// # Arguments
    ///
    /// * `x` - The augend
    /// * `y` - The addend
    ///
    /// # Returns
    ///
    /// * `z` - The sum of x and y
    ///
    /// # Panics
    ///
    /// Panics if overflow or underflow occurs during the addition.
    pub fn add_signed(x: i128, y: i128) -> i128 {
        let z = x.checked_add(y).expect("Signed addition overflow");
        z
    }

    /// Returns x - y, panics if overflows or underflows i128
    ///
    /// # Arguments
    ///
    /// * `x` - The minuend
    /// * `y` - The subtrahend
    ///
    /// # Returns
    ///
    /// * `z` - The difference of x and y
    ///
    /// # Panics
    ///
    /// Panics if overflow or underflow occurs during the subtraction.
    pub fn sub_signed(x: i128, y: i128) -> i128 {
        let z = x.checked_sub(y).expect("Signed subtraction underflow");
        z
    }
}

fn main() {
    // Example usage:
    let x: u128 = 100;
    let y: u128 = 50;
    let z = low_gas_safe_math::add(x, y);
    println!("Result: {}", z); // This would print 150
}
