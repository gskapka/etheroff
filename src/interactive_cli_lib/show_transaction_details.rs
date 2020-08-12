use crate::{
    lib::{
        types::Result,
        constants::ONE_ETH_IN_WEI,
    },
    interactive_cli_lib::state::InteractiveCliState,
};

pub fn show_transaction_details(state: InteractiveCliState) -> Result<InteractiveCliState> {
    let amount_in_wei = &state.wei_amount?;
    let gas_limit_wei = &state.eth_gas_limit?;
    let gas_price_in_wei = &state.eth_gas_price_wei?;
    let total_cost_in_wei = amount_in_wei.saturating_add(gas_limit_wei.saturating_mul(*gas_price_in_wei));
    let total_cost_in_eth = total_cost_in_wei.as_u64() as f64 / ONE_ETH_IN_WEI as f64;
    println!("\n❍ Transaction details:");
    println!("To:         {}", &state.eth_address?);
    println!("Amount:     {} Ξ ({} Wei)", &state.eth_amount.clone()?, amount_in_wei);
    println!("Nonce:      {}", &state.eth_nonce?);
    println!("Gas limit:  {}", gas_limit_wei);
    println!("Gas price:  {} GWei ({} in Wei)", &state.eth_gas_price_gwei?, gas_price_in_wei);
    println!("Network:    {}", &state.eth_network?);
    println!("Calldata:   {}", format!("0x{}", hex::encode(&state.eth_calldata.clone()?)));
    println!("Total cost: {} Ξ ({} Wei)", total_cost_in_eth, total_cost_in_wei);
    Ok(state)
}
