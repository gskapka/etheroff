use tiny_keccak::keccak256;
use ethereum_types::Address as EthAddress;
use crate::lib::{
    types::Result,
    utils::{
        maybe_strip_hex_prefix,
        decode_hex_with_err_msg,
        validate_eth_private_key_hex_length,
    },
};
use secp256k1::{
    Secp256k1,
    key::{
        SecretKey,
        PublicKey
    },
};

pub struct EthereumKeys {
    private_key: SecretKey,
    pub address: EthAddress,
    pub address_string: String,
}

impl EthereumKeys {
    fn get_public_key_from_private_key(private_key: &SecretKey) -> PublicKey {
        PublicKey::from_secret_key(&Secp256k1::new(), private_key)
    }

    fn public_key_to_eth_address(public_key: &PublicKey) -> EthAddress {
        // NOTE: Need the last 20 bytes of the hash of the uncompresed form of the public key, minus it's prefix byte.
        EthAddress::from_slice(&keccak256(&public_key.serialize_uncompressed()[1..])[12..])
    }

    pub fn to_address(&self) -> EthAddress {
        Self::public_key_to_eth_address(&Self::get_public_key_from_private_key(&self.private_key))
    }

    pub fn from_private_key(private_key: &SecretKey) -> Self {
        let address = Self::public_key_to_eth_address(&Self::get_public_key_from_private_key(private_key));
        EthereumKeys {
            address,
            private_key: *private_key,
            address_string: hex::encode(&address),
        }
    }

    pub fn from_hex_private_key(hex: &str) -> Result<Self> {
        maybe_strip_hex_prefix(hex)
            .and_then(|hex_no_prefix| validate_eth_private_key_hex_length(&hex_no_prefix))
            .and_then(|valid_hex| decode_hex_with_err_msg(&valid_hex, "✘ Error decoding ETH private key hex!"))
            .and_then(|bytes| Ok(SecretKey::from_slice(&bytes)?))
            .map(|private_key| Self::from_private_key(&private_key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::key::SecretKey;

    fn get_sample_private_key_hex() -> String {
        "decaffb75a41481965e391fb6d4406b6c356d20194c5a88935151f0513c0ffee".to_string()
    }

    fn get_sample_private_key() -> SecretKey {
        SecretKey::from_slice(&hex::decode(&get_sample_private_key_hex()).unwrap()).unwrap()
    }

    #[test]
    fn should_create_etherem_keys_from_private_key() {
        let expected_address = "3eea9f85661bac934637b8407f9361caa14f5163";
        let pk = get_sample_private_key();
        let result = EthereumKeys::from_private_key(&pk);
        assert_eq!(result.address_string, expected_address);
    }
}
