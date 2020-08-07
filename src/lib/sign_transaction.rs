use crate::lib::{
    state::State,
    types::Result,
    get_cli_args::CliArgs,
    get_eth_private_key::get_eth_private_key_and_add_to_state,
};
use serde_json::{
    json,
    Value as JsonValue,
};

pub fn sign_transaction(cli_args: CliArgs) -> Result<JsonValue> {
    info!("✔ Signing transaction...");
    State::init_from_cli_args(cli_args)
        .and_then(get_eth_private_key_and_add_to_state)
        .map(|_| json!({ "result": "success", }))
}
