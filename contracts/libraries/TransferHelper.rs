// SPDX-License-Identifier: GPL-2.0-or-later

use ethers::core::types::{Address, U256};
use ethers::prelude::*;
use std::convert::TryFrom;

/// Transfers tokens from the caller to a recipient.
/// Calls transfer on token contract, panics if transfer fails
/// @param token The contract address of the token which will be transferred
/// @param to The recipient of the transfer
/// @param value The value of the transfer
fn safe_transfer(
    token: Address,
    to: Address,
    value: U256,
    provider: &Provider<Http>,
    signer: &Signer,
) -> Result<(), Box<dyn std::error::Error>> {
    // Prepare the contract instance for ERC20 token
    let contract = Contract::new(token, erc20_abi(), provider.clone());

    // Encode the transfer function call
    let data = contract
        .encode_function("transfer", (to, value))
        .expect("encoding failed");

    // Send the transaction
    let pending_tx = contract
        .send_transaction(
            TransactionRequest::new()
                .to(contract.address())
                .data(Bytes(data)),
            signer,
        )
        .await?;

    // Await the transaction confirmation
    pending_tx.await?;

    Ok(())
}

/// Example usage:
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    let signer = LocalWallet::new(&mut rand::thread_rng());

    let token_address = "0x1234567890123456789012345678901234567890".parse()?;
    let recipient = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef".parse()?;
    let amount = U256::from(100);

    safe_transfer(token_address, recipient, amount, &provider, &signer).await?;

    println!("Transfer successful!");
    Ok(())
}
