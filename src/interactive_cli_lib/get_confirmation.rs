use crate::{
    lib::{
        types::Result,
        errors::AppError,
    },
    interactive_cli_lib::{
        utils::get_user_input,
        state::InteractiveCliState,
    }
};

pub fn get_confirmation_from_user(state: InteractiveCliState) -> Result<InteractiveCliState> {
    println!("\n❍ Sign transaction? (y/n)");
    get_user_input()
        .and_then(|user_input| match &user_input.chars().count() {
            0 => Ok(state.set_should_sign_tx_to_true()),
            _ => match user_input == "y" || user_input == "yes" {
                true => Ok(state.set_should_sign_tx_to_true()),
                false => Err(AppError::Custom("✘ Okay, not signing transaction. Bye!".to_string()))
            }
        })
}
