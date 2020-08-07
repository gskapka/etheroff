use docopt::Docopt;
use crate::lib::{
    types::Result,
    errors::AppError,
    usage_info::USAGE_INFO,
};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CliArgs {
    pub cmd_version: bool,
    pub flag_chainId: usize,
    pub flag_keyfile: String,
    pub flag_logLevel: String,
    pub cmd_signTransaction: bool,
}

pub fn get_cli_args() -> Result<CliArgs> {
    match Docopt::new(USAGE_INFO)
        .and_then(|d| d.deserialize()) {
            Ok(cli_args) => Ok(cli_args),
            Err(e) => Err(AppError::Custom(e.to_string()))
        }
}
