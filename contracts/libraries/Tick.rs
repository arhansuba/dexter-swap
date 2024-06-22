// SPDX-License-Identifier: BUSL-1.1

mod low_gas_safe_math;
mod safe_cast;

use low_gas_safe_math::*;
use safe_cast::*;

/// Represents tick information
pub struct TickInfo {
    /// the total position liquidity that references this tick
    liquidity_gross: u128,
    /// amount of net liquidity added (subtracted) when tick is crossed from left to right (right to left),
    liquidity_net: i128,
    /// fee growth per unit of liquidity on the _other_ side of this tick (relative to the current tick)
    fee_growth_outside0_x128: u256,
    fee_growth_outside1_x128: u256,
    /// the cumulative tick value on the other side of the tick
    tick_cumulative_outside: i56,
    /// the seconds per unit of liquidity on the _other_ side of this tick (relative to the current tick)
    seconds_per_liquidity_outside_x128: u160,
    /// the seconds spent on the other side of the tick (relative to the current tick)
    seconds_outside: u32,
    /// true iff the tick is initialized, i.e. the value is exactly equivalent to the expression liquidity_gross != 0
    initialized: bool,
}

impl TickInfo {
    /// Creates a new tick info instance
    pub fn new() -> Self {
        TickInfo {
            liquidity_gross: 0,
            liquidity_net: 0,
            fee_growth_outside0_x128: 0,
            fee_growth_outside1_x128: 0,
            tick_cumulative_outside: 0,
            seconds_per_liquidity_outside_x128: 0,
            seconds_outside: 0,
            initialized: false,
        }
    }
}

/// Derives max liquidity per tick from given tick spacing
/// @param tick_spacing The amount of required tick separation, realized in multiples of `tick_spacing`
/// e.g., a tick_spacing of 3 requires ticks to be initialized every 3rd tick i.e., ..., -6, -3, 0, 3, 6, ...
/// @return The max liquidity per tick
pub fn tick_spacing_to_max_liquidity_per_tick(tick_spacing: i24) -> u128 {
    let min_tick = (TickMath::MIN_TICK / tick_spacing) * tick_spacing;
    let max_tick = (TickMath::MAX_TICK / tick_spacing) * tick_spacing;
    let num_ticks = ((max_tick - min_tick) / tick_spacing) as u24 + 1;
    u128::max_value() / num_ticks as u128
}

/// Retrieves fee growth data
/// @param ticks The mapping containing all tick information for initialized ticks
/// @param tick_lower The lower tick boundary of the position
/// @param tick_upper The upper tick boundary of the position
/// @param tick_current The current tick
/// @param fee_growth_global0_x128 The all-time global fee growth, per unit of liquidity, in token0
/// @param fee_growth_global1_x128 The all-time global fee growth, per unit of liquidity, in token1
/// @return fee_growth_inside0_x128 The all-time fee growth in token0, per unit of liquidity, inside the position's tick boundaries
/// @return fee_growth_inside1_x128 The all-time fee growth in token1, per unit of liquidity, inside the position's tick boundaries
pub fn get_fee_growth_inside(
    ticks: &mut std::collections::HashMap<i24, TickInfo>,
    tick_lower: i24,
    tick_upper: i24,
    tick_current: i24,
    fee_growth_global0_x128: u256,
    fee_growth_global1_x128: u256,
) -> (u256, u256) {
    let mut fee_growth_inside0_x128 = 0;
    let mut fee_growth_inside1_x128 = 0;

    // Retrieve tick information
    let lower = ticks.get(&tick_lower).unwrap_or(&TickInfo::new());
    let upper = ticks.get(&tick_upper).unwrap_or(&TickInfo::new());

    // Calculate fee growth below
    let fee_growth_below0_x128 = if tick_current >= tick_lower {
        lower.fee_growth_outside0_x128
    } else {
        fee_growth_global0_x128 - lower.fee_growth_outside0_x128
    };
    let fee_growth_below1_x128 = if tick_current >= tick_lower {
        lower.fee_growth_outside1_x128
    } else {
        fee_growth_global1_x128 - lower.fee_growth_outside1_x128
    };

    // Calculate fee growth above
    let fee_growth_above0_x128 = if tick_current < tick_upper {
        upper.fee_growth_outside0_x128
    } else {
        fee_growth_global0_x128 - upper.fee_growth_outside0_x128
    };
    let fee_growth_above1_x128 = if tick_current < tick_upper {
        upper.fee_growth_outside1_x128
    } else {
        fee_growth_global1_x128 - upper.fee_growth_outside1_x128
    };

    fee_growth_inside0_x128 = fee_growth_global0_x128 - fee_growth_below0_x128 - fee_growth_above0_x128;
    fee_growth_inside1_x128 = fee_growth_global1_x128 - fee_growth_below1_x128 - fee_growth_above1_x128;

    (fee_growth_inside0_x128, fee_growth_inside1_x128)
}

/// Updates a tick and returns true if the tick was flipped from initialized to uninitialized, or vice versa
/// @param ticks The mapping containing all tick information for initialized ticks
/// @param tick The tick that will be updated
/// @param tick_current The current tick
/// @param liquidity_delta A new amount of liquidity to be added (subtracted) when tick is crossed from left to right (right to left)
/// @param fee_growth_global0_x128 The all-time global fee growth, per unit of liquidity, in token0
/// @param fee_growth_global1_x128 The all-time global fee growth, per unit of liquidity, in token1
/// @param seconds_per_liquidity_cumulative_x128 The all-time seconds per max(1, liquidity) of the pool
/// @param tick_cumulative The tick * time elapsed since the pool was first initialized
/// @param time The current block timestamp
/// @param upper true for updating a position's upper tick, or false for updating a position's lower tick
/// @param max_liquidity The maximum liquidity allocation for a single tick
/// @return flipped Whether the tick was flipped from initialized to uninitialized, or vice versa
pub fn update_tick(
    ticks: &mut std::collections::HashMap<i24, TickInfo>,
    tick: i24,
    tick_current: i24,
    liquidity_delta: i128,
    fee_growth_global0_x128: u256,
    fee_growth_global1_x128: u256,
    seconds_per_liquidity_cumulative_x128: u160,
    tick_cumulative: i56,
    time: u32,
    upper: bool,
    max_liquidity: u128,
) -> bool {
    let info = ticks.entry(tick).or_insert(TickInfo::new());

    let liquidity_gross_before = info.liquidity_gross;
    let liquidity_gross_after = liquidity_gross_before.checked_add(liquidity_delta as u128).unwrap_or(0);

    assert!(liquidity_gross_after <= max_liquidity, "LO");

    let flipped = (liquidity_gross_after == 0) != (liquidity_gross_before == 0);

    if liquidity_gross_before == 0 {
        // By convention, we assume that all growth before a tick was initialized happened _below_ the tick
        if tick <= tick_current {
            info.fee_growth_outside0_x128 = fee_growth_global0_x128;
            info.fee_growth_outside1_x128 = fee_growth_global1_x128;
            info.seconds_per_liquidity_outside_x128 = seconds_per_liquidity_cumulative_x128;
            info.tick_cumulative_outside = tick_cumulative;
            info.seconds_outside = time;
        }
        info.initialized = true;
    }

    info.liquidity_gross = liquidity_gross_after;

    // When the lower (upper) tick is crossed left to right (right to left), liquidity must be added (removed)
    info.liquidity_net = if upper {
        info.liquidity_net - liquidity_delta
    } else {
        info.liquidity_net + liquidity_delta
    };

    flipped
}

/// Clears tick data
/// @param ticks The mapping containing all initialized tick information for initialized ticks
/// @param tick The tick that will be cleared
pub fn clear_tick(ticks: &mut std::collections::HashMap<i24, TickInfo>, tick: i24) {
    ticks.remove(&tick);
}

/// Transitions to next tick as needed by price movement
/// @param ticks The mapping containing all tick information for initialized ticks
/// @param tick The destination tick of the transition
/// @param fee_growth_global0_x128 The all-time global fee growth, per unit of liquidity, in token0
/// @param fee_growth_global1_x128 The all-time global fee growth, per unit of liquidity, in token1
/// @param seconds_per_liquidity_cumulative_x128 The current seconds per liquidity
/// @param tick_cumulative The tick * time elapsed since the pool was first initialized
/// @param time The current block.timestamp
/// @return liquidity_net The amount of liquidity added (subtracted) when tick is crossed from left to right (right to left)
pub fn cross_tick(
    ticks: &mut std::collections::HashMap<i24, TickInfo>,
    tick: i24,
    fee_growth_global0_x128: u256,
    fee_growth_global1_x128: u256,
    seconds_per_liquidity_cumulative_x128: u160,
    tick_cumulative: i56,
    time: u32,
) -> i128 {
    let info = ticks.get_mut(&tick).unwrap_or(&mut TickInfo::new());

    info.fee_growth_outside0_x128 = fee_growth_global0_x128 - info.fee_growth_outside0_x128;
    info.fee_growth_outside1_x128 = fee_growth_global1_x128 - info.fee_growth_outside1_x128;
    info.seconds_per_liquidity_outside_x128 =
        seconds_per_liquidity_cumulative_x128 - info.seconds_per_liquidity_outside_x128;
    info.tick_cumulative_outside = tick_cumulative - info.tick_cumulative_outside;
    info.seconds_outside = time - info.seconds_outside;

    info.liquidity_net
}

fn main() {
    // Example usage of the Tick library functions
    let mut ticks: std::collections::HashMap<i24, TickInfo> = std::collections::HashMap::new();
    let tick_lower: i24 = -10;
    let tick_upper: i24 = 10;
    let tick_current: i24 = 0;
    let fee_growth_global0_x128: u256 = 123456;
    let fee_growth_global1_x128: u256 = 654321;
    let max_liquidity: u128 = 1000000;

    // Example usage of functions
    let (fee_growth_inside0_x128, fee_growth_inside1_x128) = get_fee_growth_inside(
        &mut ticks,
        tick_lower,
        tick_upper,
        tick_current,
        fee_growth_global0_x128,
        fee_growth_global1_x128,
    );

    println!("Fee growth inside 0: {}", fee_growth_inside0_x128);
    println!("Fee growth inside 1: {}", fee_growth_inside1_x128);

    let flipped = update_tick(
        &mut ticks,
        tick_lower,
        tick_current,
        100,
        fee_growth_global0_x128,
        fee_growth_global1_x128,
        50,
        12345,
        1000,
        true,
        max_liquidity,
    );

    println!("Tick updated. Flipped: {}", flipped);

    clear_tick(&mut ticks, tick_lower);

    let liquidity_net = cross_tick(
        &mut ticks,
        tick_upper,
        fee_growth_global0_x128,
        fee_growth_global1_x128,
        75,
        54321,
        2000,
    );

    println!("Liquidity net: {}", liquidity_net);
}
