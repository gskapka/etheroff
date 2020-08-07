use crate::lib::{
    types::Result,
    errors::AppError,
    get_cli_args::CliArgs,
    chain_id::EthereumChainId,
    ethereum_keys::EthereumKeys,
};

fn get_no_overwrite_state_err(substring: &str) -> String {
    format!("✘ Cannot overwrite {} in state!" , substring)
}

fn get_not_in_state_err(substring: &str) -> String {
    format!("✘ No {} in state!" , substring)
}

pub struct State {
    pub cli_args: CliArgs,
    pub chain_id: EthereumChainId,
    pub eth_pk: Option<EthereumKeys>,
}

impl State {
    pub fn init_from_cli_args(cli_args: CliArgs) -> Result<State> {
        Ok(State {
            eth_pk: None,
            chain_id: EthereumChainId::from_int(&cli_args.flag_chainId)?,
            cli_args,
        })
    }
 }

impl State {
    pub fn add_eth_pk(mut self, eth_pk: EthereumKeys) -> Result<State> {
        match self.eth_pk {
            Some(_) => Err(AppError::Custom(get_no_overwrite_state_err("eth_private_key"))),
            None => {
                self.eth_pk = Some(eth_pk);
                Ok(self)
            }
        }
    }

    pub fn get_eth_pk(&self) -> Result<&EthereumKeys> {
        match &self.eth_pk {
            Some(eth_pk) => Ok(&eth_pk),
            None => Err(AppError::Custom(get_not_in_state_err("eth_pk")))
        }
    }
}
