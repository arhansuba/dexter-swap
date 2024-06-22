// SPDX-License-Identifier: BUSL-1.1

use crate::full_math::*;
use crate::fixed_point128::*;
use crate::liquidity_math::*;

/// Struct for storing position information
#[derive(Debug, Clone)]
pub struct PositionInfo {
    /// The amount of liquidity owned by this position
    pub liquidity: u128,
    /// Fee growth per unit of liquidity as of the last update to liquidity or fees owed
    pub fee_growth_inside0_last_x128: u256,
    pub fee_growth_inside1_last_x128: u256,
    /// The fees owed to the position owner in token0/token1
    pub tokens_owed0: u128,
    pub tokens_owed1: u128,
}

/// Library for managing user positions
pub mod position {
    use super::*;

    /// Returns the position info of a given owner and tick boundaries
    pub fn get(
        self_: &mut std::collections::HashMap<[u8; 32], PositionInfo>,
        owner: address,
        tick_lower: i24,
        tick_upper: i24,
    ) -> &mut PositionInfo {
        self_.entry(keccak256(abi.encode_packed(&owner, tick_lower, tick_upper)))
            .or_insert_with(|| PositionInfo {
                liquidity: 0,
                fee_growth_inside0_last_x128: 0.into(),
                fee_growth_inside1_last_x128: 0.into(),
                tokens_owed0: 0,
                tokens_owed1: 0,
            })
    }

    /// Updates the position with accumulated fees
    pub fn update(
        self_: &mut PositionInfo,
        liquidity_delta: i128,
        fee_growth_inside0_x128: u256,
        fee_growth_inside1_x128: u256,
    ) {
        let mut _self = self_.clone();

        let liquidity_next: u128;
        if liquidity_delta == 0 {
            assert!(_self.liquidity > 0); // disallow pokes for 0 liquidity positions
            liquidity_next = _self.liquidity;
        } else {
            liquidity_next = add_delta(_self.liquidity, liquidity_delta);
        }

        // calculate accumulated fees
        let tokens_owed0 = {
            let growth_delta = fee_growth_inside0_x128 - _self.fee_growth_inside0_last_x128;
            let liquidity = _self.liquidity;
            (growth_delta * liquidity / Q128).into()
        };

        let tokens_owed1 = {
            let growth_delta = fee_growth_inside1_x128 - _self.fee_growth_inside1_last_x128;
            let liquidity = _self.liquidity;
            (growth_delta * liquidity / Q128).into()
        };

        // update the position
        if liquidity_delta != 0 {
            self_.liquidity = liquidity_next;
        }
        self_.fee_growth_inside0_last_x128 = fee_growth_inside0_x128;
        self_.fee_growth_inside1_last_x128 = fee_growth_inside1_x128;
        if tokens_owed0 > 0 || tokens_owed1 > 0 {
            // overflow is acceptable, have to withdraw before you hit type(u128).max fees
            self_.tokens_owed0 += tokens_owed0;
            self_.tokens_owed1 += tokens_owed1;
        }
    }
}

/// Helper function to simulate keccak256 hashing
fn keccak256(data: &[u8]) -> [u8; 32] {
    // Simulated keccak256 hashing
    let mut hasher = sha3::Keccak256::new();
    hasher.update(data);
    let mut output = [0u8; 32];
    output.copy_from_slice(&hasher.finalize());
    output
}

/// Helper function for asserting conditions
fn assert(condition: bool) {
    if !condition {
        panic!("Assertion failed");
    }
}

/// Constants and types specific to the FixedPoint128 implementation
mod fixed_point128 {
    pub type u256 = u128;
    pub const Q128: u128 = 2_u128.pow(128);
}

/// Simulation of FullMath library with only necessary functionality
mod full_math {
    pub fn mul_div(x: u256, y: u128, z: u128) -> u256 {
        // Actual implementation of mul_div in FullMath is not provided here
        // Placeholder function for demonstration
        (x * y / z).into()
    }
}

/// Simulation of LiquidityMath library with only necessary functionality
mod liquidity_math {
    pub fn add_delta(liquidity: u128, delta: i128) -> u128 {
        // Actual implementation of add_delta in LiquidityMath is not provided here
        // Placeholder function for demonstration
        (liquidity as i128 + delta).max(0) as u128
    }
}

/// Simulation of an address type
type address = [u8; 32];

/// Main function demonstrating usage of the Position library
fn main() {
    // Example usage:
    let mut positions: std::collections::HashMap<[u8; 32], PositionInfo> = Default::default();

    let owner = [0u8; 32];
    let tick_lower = -500;
    let tick_upper = 500;

    let mut position = position::get(&mut positions, owner, tick_lower, tick_upper);

    let liquidity_delta = 100;
    let fee_growth_inside0_x128 = 200.into();
    let fee_growth_inside1_x128 = 300.into();

    position::update(
        &mut position,
        liquidity_delta,
        fee_growth_inside0_x128,
        fee_growth_inside1_x128,
    );

    println!("Position after update: {:?}", position);
}
