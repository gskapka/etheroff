use tiny_keccak::keccak256;
use ethereum_types::{
    H256,
    Address as EthAddress,
};
use secp256k1::{
    Message,
    Secp256k1,
    key::{
        ONE_KEY,
        SecretKey,
        PublicKey,
    },
};
use crate::lib::{
    types::{
        Byte,
        Result,
    },
    utils::{
        keccak_hash_bytes,
        maybe_strip_hex_prefix,
        decode_hex_with_err_msg,
        validate_eth_private_key_hex_length,
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

    pub fn sign_message_bytes(&self, message: &[Byte]) -> Result<[u8; 65]> {
        self.sign_hash(keccak_hash_bytes(message))
    }

    pub fn sign_hash(&self, hash: H256) -> Result<[u8; 65]> {
        let msg = match Message::from_slice(hash.as_bytes()) {
            Ok(msg) => msg,
            Err(err) => return Err(err.into()),
        };
        let sig = Secp256k1::sign_recoverable(&Secp256k1::new(), &msg, &self.private_key);
        let (rec_id, data) = sig.serialize_compact();
        let mut data_arr = [0; 65];
        data_arr[0..64].copy_from_slice(&data[0..64]);
        data_arr[64] = rec_id.to_i32() as u8;
        Ok(data_arr)
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
