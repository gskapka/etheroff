use ethereum_types::{
    U256,
    Address as EthAddress,
};
use crate::lib::{
    types::{
        Bytes,
        Result,
    },
    errors::AppError,
    get_cli_args::CliArgs,
    chain_id::EthereumChainId,
    ethereum_keys::EthereumKeys,
    utils::{
        maybe_pad_hex,
        maybe_strip_hex_prefix,
        decode_hex_with_err_msg,
        convert_dec_str_to_u256_with_err_msg,
    },
    ethereum_address::get_ethereum_address_from_hex_string,
};

fn get_no_overwrite_state_err(substring: &str) -> String {
    format!("✘ Cannot overwrite {} in state!" , substring)
}

fn get_not_in_state_err(substring: &str) -> String {
    format!("✘ No {} in state!" , substring)
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
    pub fn init_from_cli_args(cli_args: CliArgs) -> Result<State> {
        Ok(State {
            eth_pk: None,
            to: get_ethereum_address_from_hex_string(&cli_args.arg_to)?,
            chain_id: EthereumChainId::from_int(&cli_args.flag_chainId)?,
            data: maybe_strip_hex_prefix(&cli_args.flag_data)
                .map(|hex_no_prefix| maybe_pad_hex(&hex_no_prefix))
                .and_then(|padded_hex|
                    decode_hex_with_err_msg(
                        &padded_hex,
                        &format!("✘ Could not parse arg of '{}' to a bytes correctly!", &cli_args.flag_data),
                    )
                )?,
            nonce: convert_dec_str_to_u256_with_err_msg(
                &cli_args.arg_nonce,
                &format!("✘ Could not parse arg of '{}' to a U256 nonce correctly!", &cli_args.arg_nonce),
            )?,
            value: convert_dec_str_to_u256_with_err_msg(
                &cli_args.arg_value,
                &format!("✘ Could not parse arg of '{}' to a U256 value correctly!", &cli_args.arg_value),
            )?,
            gas_limit: convert_dec_str_to_u256_with_err_msg(
                &cli_args.flag_gasLimit,
                &format!("✘ Could not parse arg of '{}' to a U256 gas limit correctly!", &cli_args.flag_gasLimit),
            )?,
            gas_price: convert_dec_str_to_u256_with_err_msg(
                &cli_args.flag_gasPrice,
                &format!("✘ Could not parse arg of '{}' to a U256 gas price correctly!", &cli_args.flag_gasPrice),
            )?,
            cli_args,
        })
    }
}

impl State {
    pub fn add_eth_private_key(mut self, eth_pk: EthereumKeys) -> Result<State> {
        match self.eth_pk {
            Some(_) => Err(AppError::Custom(get_no_overwrite_state_err("eth_private_key"))),
            None => {
                self.eth_pk = Some(eth_pk);
                Ok(self)
            }
        }
    }

    pub fn get_eth_private_key(&self) -> Result<&EthereumKeys> {
        match &self.eth_pk {
            Some(eth_pk) => Ok(&eth_pk),
            None => Err(AppError::Custom(get_not_in_state_err("eth_pk")))
        }
    }
}
