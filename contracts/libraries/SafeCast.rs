// SPDX-License-Identifier: GPL-2.0-or-later

/// Library for safe casting methods
pub mod safe_cast {
    /// Cast a u256 to a u160, panic on overflow
    pub fn to_u160(y: u256) -> u160 {
        assert!(y <= u256::from(u160::max_value()));
        y.into()
    }

    /// Cast an i256 to an i128, panic on overflow or underflow
    pub fn to_i128(y: i256) -> i128 {
        assert!(y >= i256::from(i128::min_value()) && y <= i256::from(i128::max_value()));
        y.into()
    }

    /// Cast a u256 to an i256, panic on overflow
    pub fn to_i256(y: u256) -> i256 {
        assert!(y < u256::from(2).pow(255));
        y.into()
    }
}

/// Simulation of u256 type
type u256 = u128;

/// Simulation of u160 type
type u160 = u128;

/// Simulation of i256 type
type i256 = i128;

/// Simulation of i128 type
type i128 = i64;

/// Main function demonstrating usage of the SafeCast library
fn main() {
    let y: u256 = 123456789;
    let z_u160: u160 = safe_cast::to_u160(y);
    let z_i128: i128 = safe_cast::to_i128(-123456789);
    let z_i256: i256 = safe_cast::to_i256(y);

    println!("y (u256): {}", y);
    println!("z_u160 (u160): {}", z_u160);
    println!("z_i128 (i128): {}", z_i128);
    println!("z_i256 (i256): {}", z_i256);
}
