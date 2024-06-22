// SPDX-License-Identifier: GPL-2.0-or-later
use cosmwasm_std::{Binary, Uint256, Env, DepsMut, MessageInfo, Response, StdResult};

pub trait IXionFlashCallback {
    fn xion_flash_callback(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        fee0: Uint256,
        fee1: Uint256,
        data: Binary,
    ) -> StdResult<Response>;
}
