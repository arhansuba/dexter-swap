// SPDX-License-Identifier: GPL-2.0-or-later

/// Computes the ceiling of the division x / y.
/// @param x The dividend
/// @param y The divisor
/// @return The quotient, ceil(x / y)
fn div_rounding_up(x: u256, y: u256) -> u256 {
    x / y + if x % y > 0 { 1 } else { 0 }
}
