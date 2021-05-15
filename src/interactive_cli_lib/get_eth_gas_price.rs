use crate::{
    interactive_cli_lib::{state::InteractiveCliState, utils::get_user_input},
    lib::types::Result,
};

pub fn get_eth_gas_price_from_user(state: InteractiveCliState) -> Result<InteractiveCliState> {
    println!("❍ Please enter the gas price for the transaction (in GWEI):");
    get_user_input().and_then(|user_input| match user_input.parse::<f64>() {
        Ok(amount) => {
            println!("✔ Gas price: {} GWei", &amount);
            state.add_eth_gas_price_gwei(amount)
        },
        Err(_) => {
            println!("✘ Could not parse '{}' to an amount!", user_input);
            get_eth_gas_price_from_user(state)
        },
    })
}
