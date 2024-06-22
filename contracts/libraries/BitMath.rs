/// SPDX-License-Identifier: GPL-2.0-or-later
/// This library provides functionality for computing bit properties of an unsigned integer

use cosmwasm_std::StdError;
use cosmwasm_std::Uint128;

pub struct BitMath;

impl BitMath {
    /// Returns the index of the most significant bit of the number,
    /// where the least significant bit is at index 0 and the most significant bit is at index 255
    /// The function satisfies the property:
    /// x >= 2**mostSignificantBit(x) and x < 2**(mostSignificantBit(x)+1)
    pub fn most_significant_bit(x: Uint128) -> Result<u8, StdError> {
        if x == 0 {
            return Err(StdError::generic_err("Input must be greater than 0"));
        }

        let mut r: u8 = 0;
        let mut x = x.u128();
        
        if x >= 0x100000000000000000000000000000000 {
            x >>= 128;
            r += 128;
        }
        if x >= 0x10000000000000000 {
            x >>= 64;
            r += 64;
        }
        if x >= 0x100000000 {
            x >>= 32;
            r += 32;
        }
        if x >= 0x10000 {
            x >>= 16;
            r += 16;
        }
        if x >= 0x100 {
            x >>= 8;
            r += 8;
        }
        if x >= 0x10 {
            x >>= 4;
            r += 4;
        }
        if x >= 0x4 {
            x >>= 2;
            r += 2;
        }
        if x >= 0x2 {
            r += 1;
        }
        
        Ok(r)
    }

    /// Returns the index of the least significant bit of the number,
    /// where the least significant bit is at index 0 and the most significant bit is at index 255
    /// The function satisfies the property:
    /// (x & 2**leastSignificantBit(x)) != 0 and (x & (2**(leastSignificantBit(x)) - 1)) == 0)
    pub fn least_significant_bit(x: Uint128) -> Result<u8, StdError> {
        if x == 0 {
            return Err(StdError::generic_err("Input must be greater than 0"));
        }

        let mut r: u8 = 255;
        let mut x = x.u128();
        
        if x & 0xffffffffffffffffffffffffffffffff == 0 {
            x >>= 128;
        } else {
            r -= 128;
        }
        if x & 0xffffffffffffffff == 0 {
            x >>= 64;
        } else {
            r -= 64;
        }
        if x & 0xffffffff == 0 {
            x >>= 32;
        } else {
            r -= 32;
        }
        if x & 0xffff == 0 {
            x >>= 16;
        } else {
            r -= 16;
        }
        if x & 0xff == 0 {
            x >>= 8;
        } else {
            r -= 8;
        }
        if x & 0xf == 0 {
            x >>= 4;
        } else {
            r -= 4;
        }
        if x & 0x3 == 0 {
            x >>= 2;
        } else {
            r -= 2;
        }
        if x & 0x1 == 0 {
            r -= 1;
        }
        
        Ok(r)
    }
}
