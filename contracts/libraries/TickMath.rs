// SPDX-License-Identifier: GPL-2.0-or-later

mod bit_math;

use bit_math::*;

/// The minimum tick that may be passed to `get_sqrt_ratio_at_tick` computed from log base 1.0001 of 2**-128
const MIN_TICK: i24 = -887272;

/// The maximum tick that may be passed to `get_sqrt_ratio_at_tick` computed from log base 1.0001 of 2**128
const MAX_TICK: i24 = -MIN_TICK;

/// The minimum value that can be returned from `get_sqrt_ratio_at_tick`.
/// Equivalent to `get_sqrt_ratio_at_tick(MIN_TICK)`
const MIN_SQRT_RATIO: u160 = 4295128739;

/// The maximum value that can be returned from `get_sqrt_ratio_at_tick`.
/// Equivalent to `get_sqrt_ratio_at_tick(MAX_TICK)`
const MAX_SQRT_RATIO: u160 = 1461446703485210103287273052203988822378723970342;

/// Calculates sqrt(1.0001^tick) * 2^96
/// Throws if |tick| > max tick
/// @param tick The input tick for the above formula
/// @return sqrt_price_x96 A Fixed point Q64.96 number representing the sqrt of the ratio of the two assets (token1/token0)
/// at the given tick
fn get_sqrt_ratio_at_tick(tick: i24) -> u160 {
    let abs_tick = if tick < 0 { (-tick) as u256 } else { tick as u256 };
    assert!(abs_tick <= MAX_TICK as u256);

    let mut ratio: u256 = if abs_tick & 0x1 != 0 { 0xfffcb933bd6fad37aa2d162d1a594001 } else { 0x100000000000000000000000000000000 };
    if abs_tick & 0x2 != 0 { ratio = (ratio * 0xfff97272373d413259a46990580e213a) >> 128; }
    if abs_tick & 0x4 != 0 { ratio = (ratio * 0xfff2e50f5f656932ef12357cf3c7fdcc) >> 128; }
    if abs_tick & 0x8 != 0 { ratio = (ratio * 0xffe5caca7e10e4e61c3624eaa0941cd0) >> 128; }
    if abs_tick & 0x10 != 0 { ratio = (ratio * 0xffcb9843d60f6159c9db58835c926644) >> 128; }
    if abs_tick & 0x20 != 0 { ratio = (ratio * 0xff973b41fa98c081472e6896dfb254c0) >> 128; }
    if abs_tick & 0x40 != 0 { ratio = (ratio * 0xff2ea16466c96a3843ec78b326b52861) >> 128; }
    if abs_tick & 0x80 != 0 { ratio = (ratio * 0xfe5dee046a99a2a811c461f1969c3053) >> 128; }
    if abs_tick & 0x100 != 0 { ratio = (ratio * 0xfcbe86c7900a88aedcffc83b479aa3a4) >> 128; }
    if abs_tick & 0x200 != 0 { ratio = (ratio * 0xf987a7253ac413176f2b074cf7815e54) >> 128; }
    if abs_tick & 0x400 != 0 { ratio = (ratio * 0xf3392b0822b70005940c7a398e4b70f3) >> 128; }
    if abs_tick & 0x800 != 0 { ratio = (ratio * 0xe7159475a2c29b7443b29c7fa6e889d9) >> 128; }
    if abs_tick & 0x1000 != 0 { ratio = (ratio * 0xd097f3bdfd2022b8845ad8f792aa5825) >> 128; }
    if abs_tick & 0x2000 != 0 { ratio = (ratio * 0xa9f746462d870fdf8a65dc1f90e061e5) >> 128; }
    if abs_tick & 0x4000 != 0 { ratio = (ratio * 0x70d869a156d2a1b890bb3df62baf32f7) >> 128; }
    if abs_tick & 0x8000 != 0 { ratio = (ratio * 0x31be135f97d08fd981231505542fcfa6) >> 128; }
    if abs_tick & 0x10000 != 0 { ratio = (ratio * 0x9aa508b5b7a84e1c677de54f3e99bc9) >> 128; }
    if abs_tick & 0x20000 != 0 { ratio = (ratio * 0x5d6af8dedb81196699c329225ee604) >> 128; }
    if abs_tick & 0x40000 != 0 { ratio = (ratio * 0x2216e584f5fa1ea926041bedfe98) >> 128; }
    if abs_tick & 0x80000 != 0 { ratio = (ratio * 0x48a170391f7dc42444e8fa2) >> 128; }

    if tick > 0 { ratio = u256::max_value() / ratio; }

    // this divides by 1 << 32 rounding up to go from a Q128.128 to a Q128.96.
    // we then downcast because we know the result always fits within 160 bits due to our tick input constraint
    // we round up in the division so get_tick_at_sqrt_ratio of the output price is always consistent
    let sqrt_price_x96 = (ratio >> 32) + if ratio % (1 << 32) == 0 { 0 } else { 1 };

    sqrt_price_x96
}

/// Calculates the greatest tick value such that get_ratio_at_tick(tick) <= ratio
/// Throws in case sqrt_price_x96 < MIN_SQRT_RATIO, as MIN_SQRT_RATIO is the lowest value get_ratio_at_tick may ever return.
/// @param sqrt_price_x96 The sqrt ratio for which to compute the tick as a Q64.96
/// @return tick The greatest tick for which the ratio is less than or equal to the input ratio
fn get_tick_at_sqrt_ratio(sqrt_price_x96: u160) -> i24 {
    assert!(sqrt_price_x96 >= MIN_SQRT_RATIO && sqrt_price_x96 < MAX_SQRT_RATIO);

    let ratio: u256 = u256::from(sqrt_price_x96) << 32;

    let mut r = ratio;
    let mut msb: u8 = 0;

    for shift in (0..=7).map(|i| i * 8) {
        if r > u256::from(1).shl(255 - shift) {
            msb |= 1 << (7 - shift);
            r >>= 1 << (7 - shift);
        }
    }

    if msb >= 128 { r = ratio >> (msb - 127); }
    else { r = ratio << (127 - msb); }

    let mut log_2: i256 = ((msb as i256 - 128) << 64);

    for shift in (0..=7).map(|i| i * 8) {
        r = (r >> 1) << 1;
        let f = r >> 128;
        log_2 |= (f << (63 - shift));
        r >>= f;
    }

    let mut log_sqrt_10001: i256 = log_2 * 255738958999603826347141; // 128.128 number

    let tick_low: i24 = (log_sqrt_10001 - 3402992956809132418596140100660247210) >> 128;
    let tick_hi: i24 = (log_sqrt_10001 + 291339464771989622907027621153398088495) >> 128;

    let mut tick: i24 = tick_low;
    if tick_low != tick_hi && get_sqrt_ratio_at_tick(tick_hi) <= sqrt_price_x96 { tick = tick_hi; }

    tick
}
