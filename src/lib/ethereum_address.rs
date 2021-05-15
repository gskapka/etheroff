use ethereum_types::Address as EthAddress;

use crate::lib::{
    types::Result,
    utils::{decode_hex_with_err_msg, maybe_strip_hex_prefix},
};

pub fn get_ethereum_address_from_hex_string(eth_address: &str) -> Result<EthAddress> {
    maybe_strip_hex_prefix(eth_address)
        .and_then(|hex_no_prefix| decode_hex_with_err_msg(&hex_no_prefix, "✘ Could not decode hex in <to> address!"))
        .map(|bytes| EthAddress::from_slice(&bytes))
}
