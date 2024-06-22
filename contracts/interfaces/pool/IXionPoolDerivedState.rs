use cosmwasm_std::{Uint32, Uint160, StdResult, QuerierWrapper};

pub trait IXionPoolDerivedState {
    fn observe(
        &self,
        querier: &QuerierWrapper,
        seconds_ago: Vec<Uint32>,
    ) -> StdResult<(Vec<i64>, Vec<Uint160>)>;

    fn snapshot_cumulatives_inside(
        &self,
        querier: &QuerierWrapper,
        tick_lower: i32,
        tick_upper: i32,
    ) -> StdResult<(i64, Uint160, u32)>;
}
