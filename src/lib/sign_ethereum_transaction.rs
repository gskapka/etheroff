use ethereum_types::{
    U256,
    Address as EthAddress,
};
use crate::lib::{
    types::{
        Bytes,
        Result,
    },
    state::State,
    get_cli_args::CliArgs,
    ethereum_transaction::EthereumTransaction,
    decrypt_ethereum_private_key::decrypt_ethereum_private_key_and_add_to_state,
};

fn sign_transaction(state: State) -> Result<String> {
    info!("✔ Signing ETH transaction...");
    info!("✔ To: {}", state.to);
    info!("✔ For amount: {} Wei", state.value);
    info!("✔ Using nonce: {} ", state.nonce);
    info!("✔ On {} ", state.chain_id);
    info!("✔ With gas limit of: {}", state.gas_limit);
    info!("✔ And a gas price of: {} Wei", state.gas_price);
    info!("✔ With calldata: 0x{}", hex::encode(&state.data));
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
