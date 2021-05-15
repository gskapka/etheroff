use crate::{
    interactive_cli_lib::state::InteractiveCliState,
    lib::{
        constants::ONE_ETH_IN_WEI,
        types::{NoneError, Result},
    },
};

pub fn show_transaction_details(state: InteractiveCliState) -> Result<InteractiveCliState> {
    let amount_in_wei = &state.wei_amount.ok_or(NoneError("Error unwrapping WEI amount"))?;
    let gas_limit_wei = &state.eth_gas_limit.ok_or(NoneError("Error unwrapping gas limit!"))?;
    let gas_price_in_wei = &state
        .eth_gas_price_wei
        .ok_or(NoneError("Error unwrapping gas price in wei!"))?;
    let total_cost_in_wei = amount_in_wei.saturating_add(gas_limit_wei.saturating_mul(*gas_price_in_wei));
    let total_cost_in_eth = total_cost_in_wei
        .checked_div(*ONE_ETH_IN_WEI)
        .ok_or(NoneError("Error getting total cost in wei!"))?;
    println!("\n❍ Transaction details:");
    println!(
        "To:         {}",
        &state.eth_address.ok_or(NoneError("Error geting ETH address!"))?
    );
    println!(
        "Amount:     {} Ξ ({} Wei)",
        &state.eth_amount.clone().ok_or(NoneError("Error getting ETH amount!"))?,
        amount_in_wei
    );
    println!(
        "Nonce:      {}",
        &state.eth_nonce.ok_or(NoneError("Error getting ETH nonce!"))?
    );
    println!("Gas limit:  {}", gas_limit_wei);
    println!(
        "Gas price:  {} GWei ({} Wei)",
        &state
            .eth_gas_price_gwei
            .ok_or(NoneError("Error getting ETH gas price!"))?,
        gas_price_in_wei
    );
    println!(
        "Network:    {}",
        &state.eth_network.ok_or(NoneError("Error getting ETH network!"))?
    );
    println!(
        "Calldata:   {}",
        format!(
            "0x{}",
            hex::encode(
                &state
                    .eth_calldata
                    .clone()
                    .ok_or(NoneError("Error getting ETH calldata!"))?
            )
        )
    );
    println!("Total cost: {} Ξ ({} Wei)", total_cost_in_eth, total_cost_in_wei);
    Ok(state)
}
