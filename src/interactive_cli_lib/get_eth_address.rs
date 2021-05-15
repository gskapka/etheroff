use ethereum_types::Address as EthAddress;

use crate::{
    interactive_cli_lib::{state::InteractiveCliState, utils::get_user_input},
    lib::{types::Result, utils::maybe_strip_hex_prefix},
};

pub const ETH_ADDRESS_CHARS_LENGTH: usize = 40; // TODO move to constants?

pub fn get_eth_address_from_user(state: InteractiveCliState) -> Result<InteractiveCliState> {
    println!("❍ Please enter the ethereum address to send to:");
    get_user_input()
        .and_then(|user_input| maybe_strip_hex_prefix(&user_input))
        .and_then(
            |hex_no_prefix| match hex_no_prefix.chars().count() == ETH_ADDRESS_CHARS_LENGTH {
                false => {
                    println!(
                        "✘ An ETH address is {} hex chars (without the `0x` prefix)!",
                        ETH_ADDRESS_CHARS_LENGTH
                    );
                    get_eth_address_from_user(state)
                },
                true => match hex::decode(&hex_no_prefix) {
                    Ok(bytes) => {
                        let address = EthAddress::from_slice(&bytes);
                        println!("✔ ETH address: {}", &address);
                        Ok(state.add_eth_address(address))
                    },
                    Err(_) => {
                        println!("✘ Error decoding that ETH address!");
                        get_eth_address_from_user(state)
                    },
                },
            },
        )
}
