use crate::lib::types::Result;

pub(crate) mod get_confirmation;
pub(crate) mod get_eth_address;
pub(crate) mod get_eth_amount;
pub(crate) mod get_eth_calldata;
pub(crate) mod get_eth_gas_limit;
pub(crate) mod get_eth_gas_price;
pub(crate) mod get_eth_network;
pub(crate) mod get_eth_nonce;
pub(crate) mod get_eth_private_key;
pub(crate) mod show_transaction_details;
pub(crate) mod sign_transaction;
pub(crate) mod state;
pub(crate) mod utils;

use crate::interactive_cli_lib::{
    get_confirmation::get_confirmation_from_user,
    get_eth_address::get_eth_address_from_user,
    get_eth_amount::get_eth_amount_from_user,
    get_eth_calldata::get_eth_calldata_from_user,
    get_eth_gas_limit::get_eth_gas_limit_from_user,
    get_eth_gas_price::get_eth_gas_price_from_user,
    get_eth_network::get_eth_network_from_user,
    get_eth_nonce::get_eth_nonce_from_user,
    get_eth_private_key::decrypt_ethereum_private_key_and_add_to_state,
    show_transaction_details::show_transaction_details,
    sign_transaction::sign_transaction,
    state::InteractiveCliState,
};

pub fn run_interactive_cli(private_key_path: String) -> Result<String> {
    println!("❍ Etheroff - An ethereum offline transaction signer!");
    decrypt_ethereum_private_key_and_add_to_state(&private_key_path, InteractiveCliState::new())
        .and_then(get_eth_address_from_user)
        .and_then(get_eth_amount_from_user)
        .and_then(get_eth_nonce_from_user)
        .and_then(get_eth_gas_limit_from_user)
        .and_then(get_eth_gas_price_from_user)
        .and_then(get_eth_network_from_user)
        .and_then(get_eth_calldata_from_user)
        .and_then(show_transaction_details)
        .and_then(get_confirmation_from_user)
        .and_then(sign_transaction)
}
