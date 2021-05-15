use crate::{
    interactive_cli_lib::{state::InteractiveCliState, utils::get_user_input},
    lib::types::Result,
};

pub fn get_eth_nonce_from_user(state: InteractiveCliState) -> Result<InteractiveCliState> {
    println!("❍ Please enter the nonce for the transaction:");
    get_user_input().and_then(|user_input| match user_input.parse::<u64>() {
        Ok(amount) => {
            println!("✔ Nonce: {}", &amount);
            Ok(state.add_eth_nonce(amount)) // TODO parse to correct type to put in state!
        },
        Err(_) => {
            println!("✘ Could not parse '{}' to an amount!", user_input);
            get_eth_nonce_from_user(state)
        },
    })
}
