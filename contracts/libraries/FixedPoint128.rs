/// SPDX-License-Identifier: GPL-2.0-or-later
/// A library for handling binary fixed point numbers, see https://en.wikipedia.org/wiki/Q_(number_format)

pub struct FixedPoint128;

impl FixedPoint128 {
    pub const Q128: u128 = 0x100000000000000000000000000000000;
}
