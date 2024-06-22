use cosmwasm_std::{Deps, StdResult};
use cosmwasm_std::Uint128;

pub trait IXionPoolState {
    fn slot0(&self, deps: Deps) -> StdResult<(Uint160, i32, u16, u16, u16, u8, bool)>;
    fn fee_growth_global0_x128(&self, deps: Deps) -> StdResult<Uint256>;
    fn fee_growth_global1_x128(&self, deps: Deps) -> StdResult<Uint256>;
    fn protocol_fees(&self, deps: Deps) -> StdResult<(Uint128, Uint128)>;
    fn liquidity(&self, deps: Deps) -> StdResult<Uint128>;
    fn ticks(
        &self,
        deps: Deps,
        tick: i32,
    ) -> StdResult<(Uint128, i128, Uint256, Uint256, i56, Uint160, u32, bool)>;
    fn tick_bitmap(&self, deps: Deps, word_position: i16) -> StdResult<Uint256>;
    fn positions(
        &self,
        deps: Deps,
        key: [u8; 32],
    ) -> StdResult<(Uint128, Uint256, Uint256, Uint128, Uint128)>;
    fn observations(
        &self,
        deps: Deps,
        index: u256,
    ) -> StdResult<(u32, i56, Uint160, bool)>;
}
