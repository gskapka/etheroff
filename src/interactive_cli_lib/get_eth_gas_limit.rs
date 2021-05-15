use ethereum_types::U256;

use crate::{
    interactive_cli_lib::{state::InteractiveCliState, utils::get_user_input},
    lib::types::Result,
};

pub fn get_eth_gas_limit_from_user(state: InteractiveCliState) -> Result<InteractiveCliState> {
    println!("❍ Please enter the gas limit for the transaction:");
    get_user_input().and_then(|user_input| match user_input.parse::<u64>() {
        Ok(amount) => {
            println!("✔ Gas limit: {}", &amount);
            Ok(state.add_eth_gas_limit(U256::from(amount)))
        },
        Err(_) => {
            println!("✘ Could not parse '{}' to an amount!", user_input);
            get_eth_gas_limit_from_user(state)
        },
    })
}
