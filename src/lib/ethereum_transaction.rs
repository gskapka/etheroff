use rlp::RlpStream;
use ethereum_types::U256;
use crate::lib::{
    ethereum_keys::EthereumKeys,
    types::{
        Byte,
        Bytes,
        Result,
    },
};

#[derive(Clone, Debug, Eq, PartialEq,)]
pub struct EthereumTransaction {
    pub v: u64,
    pub r: U256,
    pub s: U256,
    pub to: Bytes,
    pub nonce: U256,
    pub value: U256,
    pub data: Bytes,
    pub chain_id: Byte,
    pub gas_limit: U256,
    pub gas_price: U256,
}

impl EthereumTransaction {
    pub fn new_unsigned_transaction(
        to: Bytes,
        data: Bytes,
        nonce: U256,
        value: U256,
        chain_id: Byte,
        gas_limit: U256,
        gas_price: U256,
    ) -> EthereumTransaction {
        EthereumTransaction {
            to,
            data,
            nonce,
            value,
            chain_id,
            gas_limit,
            gas_price,
            r: U256::zero(),
            s: U256::zero(),
            v: chain_id.into(), // Per EIP155
        }
    }

    fn add_signature_to_transaction(mut self, sig: [u8; 65]) -> Self {
        self.r = sig[0..32].into();
        self.s = sig[32..64].into();
        self.v = Self::calculate_v_from_chain_id(sig[64], self.chain_id);
        self
    }

    fn calculate_v_from_chain_id(parity_of_y: u8, chain_id: u8) -> u64 {
        chain_id as u64 * 2 + parity_of_y as u64 + 35
    }

    pub fn serialize_bytes(&self) -> Bytes {
        let mut rlp_stream = RlpStream::new();
        rlp_stream.begin_list(9);
        rlp_stream.append(&self.nonce);
        rlp_stream.append(&self.gas_price);
        rlp_stream.append(&self.gas_limit);
        rlp_stream.append(&self.to);
        rlp_stream.append(&self.value);
        rlp_stream.append(&self.data);
        rlp_stream.append(&self.v);
        rlp_stream.append(&self.r);
        rlp_stream.append(&self.s);
        rlp_stream.out()
    }

    pub fn sign(self, eth_private_key: &EthereumKeys) -> Result<Self> {
        eth_private_key
            .sign_message_bytes(&self.serialize_bytes())
            .map(|signature| self.add_signature_to_transaction(signature))
    }

    pub fn serialize_hex(&self) -> String {
        hex::encode(self.serialize_bytes())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        get_sample_eth_address,
        get_sample_ethereum_keys,
    };

    #[test]
    fn should_sign_transaction() {
        let expected_result = "f869820539850df847580082753094fedfe2616eb3661cb8fed2782f5f0cc91d59dcac01830decaf2aa0c8a9434fa0775488d27f5395bcd4e1180b0b67800d58bc3a69f1e9071e45d3eba06084d3d8e723188c441403e0ecd6d7cd894346730a28d557eda393fc2eb0fda3";
        let pk = get_sample_ethereum_keys();
        let to = get_sample_eth_address().as_bytes().to_vec();
        let chain_id = 3u8;
        let data = hex::decode("0decaf").unwrap();
        let value: U256 = 1.into();
        let nonce: U256 = 1337.into();
        let gas_limit: U256 = 30000.into();
        let gas_price = U256::from_dec_str("60000000000").unwrap();
        let unsigned_tx = EthereumTransaction::new_unsigned_transaction(
            to,
            data,
            nonce,
            value,
            chain_id,
            gas_limit,
            gas_price,
        );
        let result = unsigned_tx.sign(&pk).unwrap().serialize_hex();
        assert_eq!(result, expected_result);
    }
}
