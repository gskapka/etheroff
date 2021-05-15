use docopt::Docopt;

use crate::lib::{errors::AppError, types::Result, usage_info::USAGE_INFO};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CliArgs {
    pub arg_to: String,
    pub arg_nonce: String,
    pub arg_value: String,
    pub cmd_version: bool,
    pub flag_data: String,
    pub flag_chainId: usize,
    pub flag_keyfile: String,
    pub flag_gasLimit: String,
    pub flag_gasPrice: String,
    pub flag_logLevel: String,
    pub cmd_signTransaction: bool,
}

pub fn get_cli_args() -> Result<CliArgs> {
    match Docopt::new(USAGE_INFO).and_then(|d| d.deserialize()) {
        Ok(cli_args) => Ok(cli_args),
        Err(e) => Err(AppError::Custom(e.to_string())),
    }
}
