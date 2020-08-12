use crate::{
    lib::{
        types::Result,
        constants::ONE_GWEI,
        ethereum_transaction::EthereumTransaction,
    },
    interactive_cli_lib::state::InteractiveCliState,
};

pub fn sign_transaction(state: InteractiveCliState) -> Result<String> {
    let gas_price_wei = state.eth_gas_price_gwei? as u64 * ONE_GWEI;
    Ok(
        EthereumTransaction::new_unsigned_transaction(
            state.eth_address?.as_bytes().to_vec(),
            state.eth_calldata.clone()?,
            state.eth_nonce?.into(),
            state.wei_amount?,
            state.eth_network?.to_byte(),
            state.eth_gas_limit?,
            gas_price_wei.into(),
        ).sign(&state.get_eth_private_key()?)?.serialize_hex()
    )
}
