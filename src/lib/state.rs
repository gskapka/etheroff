use ethereum_types::{Address as EthAddress, U256};

use crate::lib::{
    chain_id::EthereumChainId,
    ethereum_address::get_ethereum_address_from_hex_string,
    ethereum_keys::EthereumKeys,
    get_cli_args::CliArgs,
    types::{Bytes, Result},
    utils::{convert_dec_str_to_u256_with_err_msg, decode_hex_with_err_msg, maybe_pad_hex, maybe_strip_hex_prefix},
};

fn get_no_overwrite_state_err(substring: &str) -> String {
    format!("✘ Cannot overwrite {} in state!", substring)
}

fn get_not_in_state_err(substring: &str) -> String {
    format!("✘ No {} in state!", substring)
}

pub struct State {
    pub nonce: U256,
    pub value: U256,
    pub data: Bytes,
    pub to: EthAddress,
    pub gas_limit: U256,
    pub gas_price: U256,
    pub cli_args: CliArgs,
    pub chain_id: EthereumChainId,
    pub eth_pk: Option<EthereumKeys>,
}

impl State {
    pub fn init(
        eth_pk: Option<EthereumKeys>,
        to: EthAddress,
        chain_id: EthereumChainId,
        data: Bytes,
        nonce: U256,
        value: U256,
        gas_limit: U256,
        gas_price: U256,
        cli_args: CliArgs,
    ) -> State {
        State {
            nonce,
            value,
            data,
            to,
            gas_limit,
            gas_price,
            cli_args,
            chain_id,
            eth_pk,
        }
    }

    pub fn init_from_cli_args(cli_args: CliArgs) -> Result<State> {
        Ok(State::init(
            None,
            get_ethereum_address_from_hex_string(&cli_args.arg_to)?,
            EthereumChainId::from_int(&cli_args.flag_chainId)?,
            maybe_strip_hex_prefix(&cli_args.flag_data)
                .map(|hex_no_prefix| maybe_pad_hex(&hex_no_prefix))
                .and_then(|padded_hex| {
                    decode_hex_with_err_msg(
                        &padded_hex,
                        &format!(
                            "✘ Could not parse arg of '{}' to a bytes correctly!",
                            &cli_args.flag_data
                        ),
                    )
                })?,
            convert_dec_str_to_u256_with_err_msg(
                &cli_args.arg_nonce,
                &format!(
                    "✘ Could not parse arg of '{}' to a U256 nonce correctly!",
                    &cli_args.arg_nonce
                ),
            )?,
            convert_dec_str_to_u256_with_err_msg(
                &cli_args.arg_value,
                &format!(
                    "✘ Could not parse arg of '{}' to a U256 value correctly!",
                    &cli_args.arg_value
                ),
            )?,
            convert_dec_str_to_u256_with_err_msg(
                &cli_args.flag_gasLimit,
                &format!(
                    "✘ Could not parse arg of '{}' to a U256 gas limit correctly!",
                    &cli_args.flag_gasLimit
                ),
            )?,
            convert_dec_str_to_u256_with_err_msg(
                &cli_args.flag_gasPrice,
                &format!(
                    "✘ Could not parse arg of '{}' to a U256 gas price correctly!",
                    &cli_args.flag_gasPrice
                ),
            )?,
            cli_args,
        ))
    }
}

impl State {
    pub fn add_ethereum_private_key(mut self, eth_pk: EthereumKeys) -> Result<State> {
        match self.eth_pk {
            Some(_) => Err(get_no_overwrite_state_err("eth_private_key").into()),
            None => {
                self.eth_pk = Some(eth_pk);
                Ok(self)
            },
        }
    }

    pub fn get_ethereum_private_key(&self) -> Result<&EthereumKeys> {
        match &self.eth_pk {
            Some(eth_pk) => Ok(&eth_pk),
            None => Err(get_not_in_state_err("eth_pk").into()),
        }
    }
}
