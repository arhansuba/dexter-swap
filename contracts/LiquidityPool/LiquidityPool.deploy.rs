// SPDX-License-Identifier: MIT
#![cfg(test)]

use super::*;
use cosmwasm_std::{
    coin, testing::mock_dependencies, CosmosMsg, Order, Uint128, from_binary, StdError,
};
use cosmwasm_std::testing::{mock_env, mock_info};
use crate::msg::{HandleMsg, InitMsg};

#[test]
fn test_init_order_book() {
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("creator", &[]);
    
    // Initialize the contract
    let res = init(deps.as_mut(), env.clone(), info.clone());
    assert!(res.is_ok());
    
    // Verify the contract state after initialization if needed
    // Example: Check if there are no orders in the order book initially
    // let state = config(deps.as_ref()).load().expect("load state");
    // assert_eq!(state.orders.len(), 0);
}

#[test]
fn test_add_order() {
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("trader1", &[]); // Mock info of trader1
    
    // Initialize the contract (if not already initialized)
    let _res = init(deps.as_mut(), env.clone(), info.clone());

    // Add an order to the order book
    let order_id = "order1".to_string();
    let trader = "trader1".to_string();
    let token = "token".to_string();
    let amount = Uint128::new(100);
    let price = Uint128::new(10);
    let order_type = OrderType::Buy; // Example: Buy order

    let res = handle(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        HandleMsg::AddOrder {
            order_id: order_id.clone(),
            trader: trader.clone(),
            token: token.clone(),
            amount: amount.clone(),
            price: price.clone(),
            order_type: order_type.clone(),
        }
    );

    assert!(res.is_ok());

    // Retrieve and verify the order from the order book if needed
    // Example: Check if the order was added correctly
    // let orders = query_orders(deps.as_ref()).expect("load orders");
    // assert_eq!(orders.len(), 1);
    // assert_eq!(orders[0].id, order_id);
}

#[test]
fn test_match_orders() {
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("matcher", &[]); // Mock info of matcher
    
    // Initialize the contract (if not already initialized)
    let _res = init(deps.as_mut(), env.clone(), info.clone());

    // Add buy and sell orders to the order book
    let trader1 = "trader1".to_string();
    let trader2 = "trader2".to_string();
    let token = "token".to_string();
    let amount1 = Uint128::new(100);
    let amount2 = Uint128::new(80);
    let price = Uint128::new(10);

    // Add a buy order
    let res1 = handle(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        HandleMsg::AddOrder {
            order_id: "order1".to_string(),
            trader: trader1.clone(),
            token: token.clone(),
            amount: amount1.clone(),
            price: price.clone(),
            order_type: OrderType::Buy,
        }
    );

    assert!(res1.is_ok());

    // Add a sell order
    let res2 = handle(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        HandleMsg::AddOrder {
            order_id: "order2".to_string(),
            trader: trader2.clone(),
            token: token.clone(),
            amount: amount2.clone(),
            price: price.clone(),
            order_type: OrderType::Sell,
        }
    );

    assert!(res2.is_ok());

    // Match buy and sell orders
    let res3 = handle(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        HandleMsg::MatchOrders {
            token: token.clone(),
        }
    );

    assert!(res3.is_ok());

    // Verify the state after matching orders if needed
    // Example: Check if the trades were executed correctly
    // let trades = query_trades(deps.as_ref()).expect("load trades");
    // assert_eq!(trades.len(), expected_trades_count);
}
