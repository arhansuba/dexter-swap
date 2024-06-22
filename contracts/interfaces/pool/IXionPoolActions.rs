use cosmwasm_std::{Binary, Uint128, Int256, Uint160, Env, DepsMut, MessageInfo, Response, StdResult};

pub trait IXionPoolActions {
    fn initialize(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        sqrt_price_x96: Uint160,
    ) -> StdResult<Response>;

    fn mint(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        tick_lower: i32,
        tick_upper: i32,
        amount: Uint128,
        data: Binary,
    ) -> StdResult<(Uint128, Uint128)>;

    fn collect(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        tick_lower: i32,
        tick_upper: i32,
        amount0_requested: Uint128,
        amount1_requested: Uint128,
    ) -> StdResult<(Uint128, Uint128)>;

    fn burn(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        tick_lower: i32,
        tick_upper: i32,
        amount: Uint128,
    ) -> StdResult<(Uint128, Uint128)>;

    fn swap(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        zero_for_one: bool,
        amount_specified: Int256,
        sqrt_price_limit_x96: Uint160,
        data: Binary,
    ) -> StdResult<(Int256, Int256)>;

    fn flash(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        amount0: Uint128,
        amount1: Uint128,
        data: Binary,
    ) -> StdResult<Response>;

    fn increase_observation_cardinality_next(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        observation_cardinality_next: u16,
    ) -> StdResult<Response>;
}
