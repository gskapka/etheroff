use crate::{
    lib::{
        types::Result,
        decrypt_ethereum_private_key::{
            check_keyfile_exists,
            maybe_decrypt_ethereum_private_key,
            get_eth_private_key_from_hex,
        },
    },
    interactive_cli_lib::state::InteractiveCliState,
};

pub fn decrypt_ethereum_private_key_and_add_to_state(
    keyfile_path: &str,
    state: InteractiveCliState,
) -> Result<InteractiveCliState> {
    println!("❍ Decrypting ethereum private key...");
    check_keyfile_exists(&keyfile_path)
        .and_then(|_| maybe_decrypt_ethereum_private_key(keyfile_path))
        .and_then(|hex| get_eth_private_key_from_hex(&hex))
        .map(|eth_pk| state.add_ethereum_private_key(eth_pk))
}
