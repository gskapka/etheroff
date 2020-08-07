use num_format::{
    Locale,
    ToFormattedString
};
use crate::lib::{
    state::State,
    types::Result,
    get_cli_args::CliArgs,
    ethereum_transaction::EthereumTransaction,
    decrypt_ethereum_private_key::decrypt_ethereum_private_key_and_add_to_state,
};

fn sign_transaction(state: State) -> Result<String> {
    let total_eth: f64 = (state.value + (state.gas_price * state.gas_limit)).as_u64() as f64 / 1e18;
    info!("✔ Signing ETH transaction...");
    info!("✔ To: {}", state.to);
    info!("✔ On {} ", state.chain_id);
    info!("✔ Using nonce: {} ", state.nonce);
    info!("✔ For amount: {} Wei", state.value.as_u64().to_formatted_string(&Locale::en));
    info!("✔ With gas limit of: {}", state.gas_limit.as_u64().to_formatted_string(&Locale::en));
    info!("✔ And a gas price of: {} Wei", state.gas_price.as_u64().to_formatted_string(&Locale::en));
    info!("✔ With calldata: 0x{}", hex::encode(&state.data));
    info!("✔ For a total price of : {} ETH", total_eth);
    EthereumTransaction::new_unsigned_transaction(
        state.to.as_bytes().to_vec(),
        state.data.clone(),
        state.nonce,
        state.value,
        state.chain_id.to_byte(),
        state.gas_limit,
        state.gas_price,
    )
        .sign(state.get_ethereum_private_key()?)
        .map(|signed_tx| signed_tx.serialize_hex())
}

pub fn sign_ethereum_transaction(cli_args: CliArgs) -> Result<String> {
    info!("✔ Signing transaction...");
    State::init_from_cli_args(cli_args)
        .and_then(decrypt_ethereum_private_key_and_add_to_state)
        .and_then(sign_transaction)
}
