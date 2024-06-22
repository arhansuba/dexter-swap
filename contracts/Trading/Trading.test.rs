// SPDX-License-Identifier: BUSL-1.1

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{Addr, Uint128};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use crate::{TradingContract, Trade, TradeType};

    #[test]
    fn test_init_trading_contract() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("creator", &[]);

        // Initialize the contract
        let mut trading_contract = TradingContract::new(&mut deps.storage);
        let res = trading_contract.init(deps.as_mut(), env.clone(), info.clone());
        assert!(res.is_ok());

        // Verify the contract state after initialization if needed
        // Example: Check if there are no trades or accounts initially
        // let state = config(deps.as_ref()).load().expect("load state");
        // assert_eq!(state.trades.len(), 0);
        // assert_eq!(state.accounts.len(), 0);
    }

    #[test]
    fn test_handle_buy_trade() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("trader1", &[]); // Mock info of trader1

        // Initialize the contract (if not already initialized)
        let mut trading_contract = TradingContract::new(&mut deps.storage);
        let _res = trading_contract.init(deps.as_mut(), env.clone(), info.clone());

        // Add a buy trade to the trading contract
        let trade_id = "trade1".to_string();
        let trader = Addr::unchecked("trader1");
        let token = "token".to_string();
        let amount = Uint128::new(100);
        let price = Uint128::new(10);
        let trade_type = TradeType::Buy; // Example: Buy trade

        let trade = Trade {
            trade_id: trade_id.clone(),
            trader: trader.clone(),
            token: token.clone(),
            amount: amount.into(),
            price: price.into(),
            trade_type: trade_type.clone(),
        };

        let res = trading_contract.handle(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            HandleMsg::AddTrade(trade.clone()),
        );

        assert!(res.is_ok());

        // Retrieve and verify the trade from the trading contract if needed
        // Example: Check if the trade was added correctly
        // let trades = query_trades(deps.as_ref()).expect("load trades");
        // assert_eq!(trades.len(), 1);
        // assert_eq!(trades[0].trade_id, trade_id);
    }

    #[test]
    fn test_query_balances() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("query", &[]); // Mock info of query account

        // Initialize the contract (if not already initialized)
        let mut trading_contract = TradingContract::new(&mut deps.storage);
        let _res = trading_contract.init(deps.as_mut(), env.clone(), info.clone());

        // Query account balances from the trading contract
        let res = trading_contract.query_balances(
            deps.as_ref(),
            env.clone(),
            info.clone(),
            QueryMsg::GetBalances {
                trader: info.sender.clone(),
            }
        );

        assert!(res.is_ok());

        // Example: Verify the returned balances if needed
        // let balances: Vec<Account> = from_binary(&res.unwrap().data).unwrap();
        // assert_eq!(balances.len(), expected_balances_count);
    }
}
