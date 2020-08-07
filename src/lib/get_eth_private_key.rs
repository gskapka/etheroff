use std::process::Command;
use crate::lib::{
    state::State,
    types::Result,
    errors::AppError,
    ethereum_keys::EthereumKeys,
    utils::{
        file_exists,
        convert_bytes_to_string_with_no_new_lines,
    },
};

fn check_keyfile_exists(keyfile_path: &str) -> Result<()> {
    info!("✔ Checking ETH private keyfile exists...");
    match file_exists(&keyfile_path) {
        false => Err(AppError::Custom(format!("✘ ETH keyfile not found @ path: '{}'!", keyfile_path))),
        true => {
            info!("✔ Key file found @ {}!", keyfile_path);
            Ok(())
        }
    }
}

fn maybe_get_eth_private_key(keyfile_path: &str) -> Result<String> {
    info!("✔ Decrypting private key...");
    let output = Command::new("gpg").arg("-d").arg(keyfile_path).output()?;
    match output.stdout.len() {
        0 => {
            info!("✘ Error decrypting keyfile!");
            Err(AppError::Custom(convert_bytes_to_string_with_no_new_lines(&output.stderr)?))
        }
        _ => {
            info!("✔ Keyfile decrypted!");
            convert_bytes_to_string_with_no_new_lines(&output.stdout)
        }
    }
}

fn get_eth_private_key_from_hex(hex: &str) -> Result<EthereumKeys> {
    info!("✔ Creating Eth private key from hex...");
    let eth_pk = EthereumKeys::from_hex_private_key(&hex)?;
    info!("✔ ETH address: '{}'", eth_pk.to_address());
    Ok(eth_pk)
}

pub fn get_eth_private_key_and_add_to_state(state: State) -> Result<State> {
    info!("✔ Maybe getting ETH private key & adding to state...");
    let keyfile_path = &state.cli_args.flag_keyfile;
    check_keyfile_exists(keyfile_path)
        .and_then(|_| maybe_get_eth_private_key(keyfile_path))
        .and_then(|hex| get_eth_private_key_from_hex(&hex))
        .and_then(|eth_pk| state.add_eth_private_key(eth_pk))
}
