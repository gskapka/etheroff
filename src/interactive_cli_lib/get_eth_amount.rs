use crate::{
    interactive_cli_lib::{state::InteractiveCliState, utils::get_user_input},
    lib::types::Result,
};

pub fn get_eth_amount_from_user(state: InteractiveCliState) -> Result<InteractiveCliState> {
    println!("❍ Please enter the amount of ethereum to send (in ETH):");
    get_user_input()
        .and_then(|user_input| state.add_eth_amount(&user_input))
        .and_then(|state| {
            println!("✔ Amount: {} Ξ", &state.eth_amount.clone()?);
            Ok(state)
        })
}
