/// SPDX-License-Identifier: GPL-2.0-or-later
/// A library for handling binary fixed point numbers, see https://en.wikipedia.org/wiki/Q_(number_format)
/// Used in SqrtPriceMath.sol

pub struct FixedPoint96;

impl FixedPoint96 {
    pub const RESOLUTION: u8 = 96;
    pub const Q96: u128 = 0x1000000000000000000000000;
}
