use cosmwasm_std::{Deps, StdResult, Addr, Uint128, Uint24, Int24};

pub trait IXionPoolImmutables {
    fn factory(&self, deps: Deps) -> StdResult<Addr>;
    fn token0(&self, deps: Deps) -> StdResult<Addr>;
    fn token1(&self, deps: Deps) -> StdResult<Addr>;
    fn fee(&self, deps: Deps) -> StdResult<Uint24>;
    fn tick_spacing(&self, deps: Deps) -> StdResult<Int24>;
    fn max_liquidity_per_tick(&self, deps: Deps) -> StdResult<Uint128>;
}
