use crate::{
    lib::{
        types::{
            Bytes,
            Result,
        },
        chain_id::EthereumChainId,
        ethereum_keys::EthereumKeys,
    },
    interactive_cli_lib::utils::{
        convert_eth_amount_str_to_u256,
        convert_eth_gas_price_gwei_to_wei,
    },
};

use ethereum_types::{
    U256,
    Address as EthAddress,
};

pub struct InteractiveCliState {
    pub should_sign_tx: bool,
    pub eth_nonce: Option<u64>,
    pub eth_amount: Option<String>,
    pub wei_amount: Option<U256>,
    pub eth_gas_limit: Option<U256>,
    pub eth_calldata: Option<Bytes>,
    pub eth_gas_price_gwei: Option<f64>,
    pub eth_gas_price_wei: Option<U256>,
    pub eth_address: Option<EthAddress>,
    pub eth_network: Option<EthereumChainId>,
    pub eth_private_key: Option<EthereumKeys>,
}

impl InteractiveCliState {
    pub fn new() -> Self {
        InteractiveCliState {
            eth_nonce: None,
            wei_amount: None,
            eth_amount: None,
            eth_address: None,
            eth_network: None,
            eth_calldata: None,
            eth_gas_limit: None,
            eth_private_key: None,
            should_sign_tx: false,
            eth_gas_price_wei: None,
            eth_gas_price_gwei: None,
        }
    }

    pub fn set_should_sign_tx_to_true(mut self) -> Self {
        self.should_sign_tx = true;
        self
    }

    pub fn add_eth_address(mut self, eth_address: EthAddress) -> Self {
        self.eth_address = Some(eth_address);
        self
    }

    pub fn add_eth_amount(mut self, eth_amount: &str) -> Result<Self> {
        self.eth_amount = Some(eth_amount.to_string());
        self.wei_amount = Some(convert_eth_amount_str_to_u256(eth_amount)?);
        Ok(self)
    }

    pub fn add_eth_gas_limit(mut self, eth_gas_limit: U256) -> Self {
        self.eth_gas_limit = Some(eth_gas_limit);
        self
    }

    pub fn add_eth_gas_price_gwei(mut self, eth_gas_price_gwei: f64) -> Result<Self> {
        self.eth_gas_price_gwei = Some(eth_gas_price_gwei);
        self.eth_gas_price_wei = Some(convert_eth_gas_price_gwei_to_wei(eth_gas_price_gwei)?);
        Ok(self)
    }

    pub fn add_eth_nonce(mut self, eth_nonce: u64) -> Self {
        self.eth_nonce = Some(eth_nonce);
        self
    }

    pub fn add_eth_calldata(mut self, eth_calldata: Bytes) -> Self {
        self.eth_calldata = Some(eth_calldata);
        self
    }

    pub fn add_eth_network(mut self, eth_network: EthereumChainId) -> Self {
        self.eth_network = Some(eth_network);
        self
    }

    pub fn add_ethereum_private_key(mut self, eth_private_key: EthereumKeys) -> Self {
        self.eth_private_key = Some(eth_private_key);
        self
    }

    pub fn get_eth_private_key(&self) -> Result<EthereumKeys> {
        Ok(self.eth_private_key.clone()?)
    }
}
