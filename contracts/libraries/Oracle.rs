// SPDX-License-Identifier: BUSL-1.1

use std::cmp::Ordering;

/// Oracle library for storing price and liquidity observations
pub mod oracle {
    /// Struct for storing each observation
    #[derive(Debug, Clone)]
    pub struct Observation {
        block_timestamp: u32,
        tick_cumulative: i64,
        seconds_per_liquidity_cumulative_x128: u128,
        initialized: bool,
    }

    impl Observation {
        /// Constructs a new observation
        pub fn new(block_timestamp: u32) -> Self {
            Observation {
                block_timestamp,
                tick_cumulative: 0,
                seconds_per_liquidity_cumulative_x128: 0,
                initialized: true,
            }
        }
    }

    /// Transforms a previous observation into a new observation
    pub fn transform(last: &Observation, block_timestamp: u32, tick: i32, liquidity: u128) -> Observation {
        let delta = block_timestamp - last.block_timestamp;
        Observation {
            block_timestamp,
            tick_cumulative: last.tick_cumulative + (tick as i64) * delta as i64,
            seconds_per_liquidity_cumulative_x128: last.seconds_per_liquidity_cumulative_x128
                + ((delta as u128) << 128) / liquidity.max(1),
            initialized: true,
        }
    }

    /// Initializes the oracle array with the first observation
    pub fn initialize(self_: &mut [Observation; 65535], time: u32) -> (u16, u16) {
        self_[0] = Observation::new(time);
        (1, 1)
    }

    /// Writes an observation to the oracle array
    pub fn write(
        self_: &mut [Observation; 65535],
        index: u16,
        block_timestamp: u32,
        tick: i32,
        liquidity: u128,
        cardinality: u16,
        cardinality_next: u16,
    ) -> (u16, u16) {
        let last = &self_[index as usize];
        if last.block_timestamp == block_timestamp {
            return (index, cardinality);
        }

        let cardinality_updated = if cardinality_next > cardinality && index == (cardinality - 1) {
            cardinality_next
        } else {
            cardinality
        };

        let index_updated = (index + 1) % cardinality_updated;
        self_[index_updated as usize] = transform(last, block_timestamp, tick, liquidity);
        (index_updated, cardinality_updated)
    }

    /// Comparator for 32-bit timestamps
    pub fn lte(time: u32, a: u32, b: u32) -> bool {
        if a <= time && b <= time {
            return a <= b;
        }

        let a_adjusted = if a > time { a } else { a + (1 << 32) };
        let b_adjusted = if b > time { b } else { b + (1 << 32) };

        a_adjusted <= b_adjusted
    }

    /// Binary search for observations
    pub fn binary_search(
        self_: &[Observation; 65535],
        time: u32,
        target: u32,
        index: u16,
        cardinality: u16,
    ) -> (Observation, Observation) {
        let mut l = (index + 1) as usize % cardinality as usize;
        let mut r = (l + cardinality as usize - 1) % cardinality as usize;
        let mut i: usize;

        loop {
            i = (l + r) / 2;

            let before_or_at = &self_[i % cardinality as usize];
            if !before_or_at.initialized {
                l = i + 1;
                continue;
            }

            let at_or_after = &self_[(i + 1) % cardinality as usize];

            let target_at_or_after = lte(time, before_or_at.block_timestamp, target);

            if target_at_or_after && lte(time, target, at_or_after.block_timestamp) {
                break;
            }

            if !target_at_or_after {
                r = i - 1;
            } else {
                l = i + 1;
            }
        }

        (before_or_at.clone(), at_or_after.clone())
    }

    /// Retrieves surrounding observations based on a target timestamp
    pub fn get_surrounding_observations(
        self_: &[Observation; 65535],
        time: u32,
        target: u32,
        tick: i32,
        index: u16,
        liquidity: u128,
        cardinality: u16,
    ) -> (Observation, Observation) {
        let mut before_or_at = self_[index as usize].clone();

        if lte(time, before_or_at.block_timestamp, target) {
            if before_or_at.block_timestamp == target {
                return (before_or_at.clone(), before_or_at);
            } else {
                return (before_or_at.clone(), transform(&before_or_at, target, tick, liquidity));
            }
        }

        before_or_at = self_[(index + 1) as usize % cardinality as usize].clone();
        if !before_or_at.initialized {
            before_or_at = self_[0].clone();
        }

        assert!(lte(time, before_or_at.block_timestamp, target));

        binary_search(self_, time, target, index, cardinality)
    }

    /// Retrieves a single observation based on a target timestamp
    pub fn observe_single(
        self_: &[Observation; 65535],
        time: u32,
        seconds_ago: u32,
        tick: i32,
        index: u16,
        liquidity: u128,
        cardinality: u16,
    ) -> (i64, u128) {
        if seconds_ago == 0 {
            let last = &self_[index as usize];
            let last_transformed = if last.block_timestamp != time {
                transform(last, time, tick, liquidity)
            } else {
                last.clone()
            };
            return (
                last_transformed.tick_cumulative,
                last_transformed.seconds_per_liquidity_cumulative_x128,
            );
        }

        let target = time - seconds_ago;

        let (before_or_at, at_or_after) = get_surrounding_observations(
            self_,
            time,
            target,
            tick,
            index,
            liquidity,
            cardinality,
        );

        if target == before_or_at.block_timestamp {
            (before_or_at.tick_cumulative, before_or_at.seconds_per_liquidity_cumulative_x128)
        } else if target == at_or_after.block_timestamp {
            (at_or_after.tick_cumulative, at_or_after.seconds_per_liquidity_cumulative_x128)
        } else {
            let observation_time_delta = at_or_after.block_timestamp - before_or_at.block_timestamp;
            let target_delta = target - before_or_at.block_timestamp;

            let tick_cumulative = before_or_at.tick_cumulative
                + ((at_or_after.tick_cumulative - before_or_at.tick_cumulative)
                    / observation_time_delta as i64)
                    * target_delta;

            let seconds_per_liquidity_cumulative_x128 = before_or_at
                .seconds_per_liquidity_cumulative_x128
                + (((at_or_after.seconds_per_liquidity_cumulative_x128
                    - before_or_at.seconds_per_liquidity_cumulative_x128)
                    * target_delta as u128)
                    / observation_time_delta as u128);

            (tick_cumulative, seconds_per_liquidity_cumulative_x128)
        }
    }

    /// Retrieves multiple observations based on an array of seconds ago values
    pub fn observe(
        self_: &[Observation; 65535],
        time: u32,
        seconds_agos: &[u32],
        tick: i32,
        index: u16,
        liquidity: u128,
        cardinality: u16,
    ) -> (Vec<i64>, Vec<u128>) {
        let mut tick_cumulatives = Vec::with_capacity(seconds_agos.len());
        let mut seconds_per_liquidity_cumulative_x128s = Vec::with_capacity(seconds_agos.len());

        for &seconds_ago in seconds_agos.iter() {
            let (tick_cumulative, seconds_per_liquidity_cumulative_x128) = observe_single(
                self_,
                time,
                seconds_ago,
                tick,
                index,
                liquidity,
                cardinality,
            );
            tick_cumulatives.push(tick_cumulative);
            seconds_per_liquidity_cumulative_x128s.push(seconds_per_liquidity_cumulative_x128);
        }

        (tick_cumulatives, seconds_per_liquidity_cumulative_x128s)
    }
}

/// Helper function to compare timestamps for less than or equal
fn lte(time: u32, a: u32, b: u32) -> bool {
    if a <= time && b <= time {
        a <= b
    } else {
        let a_adjusted = if a > time { a } else { a + (1 << 32) };
        let b_adjusted = if b > time { b } else { b + (1 << 32) };
        a_adjusted <= b_adjusted
    }
}

/// Main function demonstrating usage of the Oracle library
fn main() {
    // Example usage:
    let mut oracle_array = [oracle::Observation::new(0); 65535];
    let (cardinality, cardinality_next) = oracle::initialize(&mut oracle_array, 1000);

    let time = 1100;
    let tick = 100;
    let liquidity = 1000;
    let index = 0;
    let cardinality = 1;
    let cardinality_next = 2;

    let (index_updated, cardinality_updated) = oracle::write(
        &mut oracle_array,
        index,
        time,
        tick,
        liquidity,
        cardinality,
        cardinality_next,
    );

    println!(
        "Updated index: {}, Updated cardinality: {}",
        index_updated, cardinality_updated
    );
}
