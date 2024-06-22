use cosmwasm_std::{Deps, StdResult, Addr, Uint128};

pub trait IXionPoolOwnerActions {
    fn set_fee_protocol(&self, deps: Deps, fee_protocol0: u8, fee_protocol1: u8) -> StdResult<()>;
    fn collect_protocol(
        &self,
        deps: Deps,
        recipient: Addr,
        amount0_requested: Uint128,
        amount1_requested: Uint128,
    ) -> StdResult<(Uint128, Uint128)>;
}
