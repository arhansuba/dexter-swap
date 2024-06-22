// SPDX-License-Identifier: BUSL-1.1

mod bit_math;

use bit_math::*;

/// Computes the position in the mapping where the initialized bit for a tick lives
/// Returns (word_pos, bit_pos)
fn position(tick: i24) -> (i16, u8) {
    let word_pos = (tick >> 8) as i16;
    let bit_pos = (tick % 256) as u8;
    (word_pos, bit_pos)
}

/// Flips the initialized state for a given tick from false to true, or vice versa
fn flip_tick(self_: &mut std::collections::HashMap<i16, u256>, tick: i24, tick_spacing: i24) {
    assert!(tick % tick_spacing == 0);
    let (word_pos, bit_pos) = position(tick / tick_spacing);
    let mask = 1 << bit_pos;
    if let Some(word) = self_.get_mut(&word_pos) {
        *word ^= mask;
    } else {
        self_.insert(word_pos, mask);
    }
}

/// Returns the next initialized tick contained in the same word (or adjacent word) as the tick that is either
/// to the left (less than or equal to) or right (greater than) of the given tick
fn next_initialized_tick_within_one_word(
    self_: &std::collections::HashMap<i16, u256>,
    tick: i24,
    tick_spacing: i24,
    lte: bool,
) -> (i24, bool) {
    let compressed = tick / tick_spacing;
    let compressed_adjusted = if tick < 0 && tick % tick_spacing != 0 {
        compressed - 1
    } else {
        compressed
    };

    if lte {
        if let Some(&word) = self_.get(&position(compressed_adjusted).0) {
            let mask = (1 << position(compressed_adjusted).1) - 1 + (1 << position(compressed_adjusted).1);
            let masked = word & mask;
            if masked != 0 {
                let next = (compressed_adjusted - (position(compressed_adjusted).1 - most_significant_bit(masked))) * tick_spacing;
                return (next, true);
            }
        }
        ((compressed_adjusted - position(compressed_adjusted).1) * tick_spacing, false)
    } else {
        let (word_pos, bit_pos) = position(compressed_adjusted + 1);
        if let Some(&word) = self_.get(&word_pos) {
            let mask = !((1 << bit_pos) - 1);
            let masked = word & mask;
            if masked != 0 {
                let next = (compressed_adjusted + 1 + (least_significant_bit(masked) - bit_pos)) * tick_spacing;
                return (next, true);
            }
        }
        ((compressed_adjusted + 1 + (u8::MAX - bit_pos)) * tick_spacing, false)
    }
}
