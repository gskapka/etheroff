#![cfg(test)]
use ethereum_types::Address as EthAddress;
use secp256k1::key::SecretKey;

use crate::lib::ethereum_keys::EthereumKeys;

pub fn get_sample_private_key_hex() -> String {
    "f6fa7f3f796e02457573f7a729457e037436812d1660a238186afc83fa5ea35e".to_string()
}

pub fn get_sample_private_key() -> SecretKey {
    SecretKey::from_slice(&hex::decode(&get_sample_private_key_hex()).unwrap()).unwrap()
}

pub fn get_sample_ethereum_keys() -> EthereumKeys {
    EthereumKeys::from_private_key(&get_sample_private_key())
}

pub fn get_sample_eth_address_hex() -> String {
    "fEDFe2616EB3661CB8FEd2782F5F0cC91D59DCaC".to_string()
}

pub fn get_sample_eth_address() -> EthAddress {
    EthAddress::from_slice(&hex::decode(get_sample_eth_address_hex()).unwrap())
}
