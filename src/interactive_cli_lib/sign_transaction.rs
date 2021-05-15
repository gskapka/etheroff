use crate::{
    interactive_cli_lib::state::InteractiveCliState,
    lib::{
        constants::ONE_GWEI,
        ethereum_transaction::EthereumTransaction,
        types::{NoneError, Result},
    },
};

pub fn sign_transaction(state: InteractiveCliState) -> Result<String> {
    let gas_price_wei = state
        .eth_gas_price_gwei
        .ok_or(NoneError("Error getting ETH gas price!"))? as u64
        * ONE_GWEI;
    Ok(EthereumTransaction::new_unsigned_transaction(
        state
            .eth_address
            .ok_or(NoneError("Error getting address!"))?
            .as_bytes()
            .to_vec(),
        state.eth_calldata.clone().ok_or(NoneError("Error getting calldata!"))?,
        state.eth_nonce.ok_or(NoneError("Error getting ETH nonce!"))?.into(),
        state.wei_amount.ok_or(NoneError("Error getting amount!"))?,
        state
            .eth_network
            .ok_or(NoneError("Error getting ETH network!"))?
            .to_byte(),
        state.eth_gas_limit.ok_or(NoneError("Error getting gas limit!"))?,
        gas_price_wei.into(),
    )
    .sign(&state.get_eth_private_key()?)?
    .serialize_hex())
}
