use cosmwasm_std::{Binary, Uint256, Env, DepsMut, MessageInfo, Response, StdResult};

pub trait IXionMintCallback {
    fn xion_mint_callback(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        amount0_owed: Uint256,
        amount1_owed: Uint256,
        data: Binary,
    ) -> StdResult<Response>;
}
