use cosmwasm_std::{Uint128, Uint160, Uint256, Binary};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InitializeEvent {
    pub sqrt_price_x96: Uint160,
    pub tick: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MintEvent {
    pub sender: String,
    pub owner: String,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub amount: Uint128,
    pub amount0: Uint256,
    pub amount1: Uint256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CollectEvent {
    pub owner: String,
    pub recipient: String,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub amount0: Uint128,
    pub amount1: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BurnEvent {
    pub owner: String,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub amount: Uint128,
    pub amount0: Uint256,
    pub amount1: Uint256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SwapEvent {
    pub sender: String,
    pub recipient: String,
    pub amount0: i128,
    pub amount1: i128,
    pub sqrt_price_x96: Uint160,
    pub liquidity: Uint128,
    pub tick: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FlashEvent {
    pub sender: String,
    pub recipient: String,
    pub amount0: Uint256,
    pub amount1: Uint256,
    pub paid0: Uint256,
    pub paid1: Uint256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IncreaseObservationCardinalityNextEvent {
    pub observation_cardinality_next_old: u16,
    pub observation_cardinality_next_new: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SetFeeProtocolEvent {
    pub fee_protocol0_old: u8,
    pub fee_protocol1_old: u8,
    pub fee_protocol0_new: u8,
    pub fee_protocol1_new: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CollectProtocolEvent {
    pub sender: String,
    pub recipient: String,
    pub amount0: Uint128,
    pub amount1: Uint128,
}

pub trait IXionPoolEvents {
    fn emit_initialize(&self, event: InitializeEvent);
    fn emit_mint(&self, event: MintEvent);
    fn emit_collect(&self, event: CollectEvent);
    fn emit_burn(&self, event: BurnEvent);
    fn emit_swap(&self, event: SwapEvent);
    fn emit_flash(&self, event: FlashEvent);
    fn emit_increase_observation_cardinality_next(&self, event: IncreaseObservationCardinalityNextEvent);
    fn emit_set_fee_protocol(&self, event: SetFeeProtocolEvent);
    fn emit_collect_protocol(&self, event: CollectProtocolEvent);
}
