use cosmwasm_std::{Binary, Int256, Env, DepsMut, MessageInfo, Response, StdResult};

pub trait IXionSwapCallback {
    fn xion_swap_callback(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        amount0_delta: Int256,
        amount1_delta: Int256,
        data: Binary,
    ) -> StdResult<Response>;
}
