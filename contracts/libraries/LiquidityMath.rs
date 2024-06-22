// SPDX-License-Identifier: GPL-2.0-or-later

/// Math library for liquidity
pub mod liquidity_math {
    /// Add a signed liquidity delta to liquidity and revert if it overflows or underflows
    ///
    /// # Arguments
    ///
    /// * `x` - The liquidity before change
    /// * `y` - The delta by which liquidity should be changed
    ///
    /// # Returns
    ///
    /// * `z` - The liquidity delta
    ///
    /// # Panics
    ///
    /// Panics if overflow or underflow occurs during the calculation.
    pub fn add_delta(x: u128, y: i128) -> u128 {
        if y < 0 {
            let neg_y = -y as u128;
            // Check for underflow
            assert!(neg_y <= x, "LS");
            x - neg_y
        } else {
            let pos_y = y as u128;
            // Check for overflow
            assert!(pos_y <= u128::MAX - x, "LA");
            x + pos_y
        }
    }
}

fn main() {
    // Example usage:
    let x: u128 = 100;
    let y: i128 = -50;
    let z = liquidity_math::add_delta(x, y);
    println!("Result: {}", z); // This would print 50
}
