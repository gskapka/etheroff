use crate::{
    interactive_cli_lib::{state::InteractiveCliState, utils::get_user_input},
    lib::{types::Result, utils::maybe_strip_hex_prefix},
};

pub fn get_eth_calldata_from_user(state: InteractiveCliState) -> Result<InteractiveCliState> {
    println!("❍ Please enter any calldata you want included (or leave blank for none!):");
    get_user_input()
        .and_then(|user_input| maybe_strip_hex_prefix(&user_input))
        .map(|hex_no_prefix| match hex_no_prefix.chars().count() % 2 {
            0 => hex_no_prefix,
            _ => format!("0{}", hex_no_prefix),
        })
        .and_then(|padded_hex| match hex::decode(&padded_hex) {
            Ok(bytes) => {
                println!("✔ ETH calldata: 0x{}", hex::encode(&bytes));
                Ok(state.add_eth_calldata(bytes))
            },
            Err(_) => {
                println!("✘ Error decoding that calldata!");
                get_eth_calldata_from_user(state)
            },
        })
}
