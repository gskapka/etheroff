#![allow(clippy::match_bool)]

extern crate docopt;
extern crate simplelog;
extern crate secp256k1;
extern crate tiny_keccak;
extern crate ethereum_types;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

mod lib;

use crate::lib::{
    types::Result,
    errors::AppError,
    usage_info::USAGE_INFO,
    sign_ethereum_transaction::sign_ethereum_transaction,
    get_tool_version_info::get_tool_version_info,
    initialize_logger::maybe_initialize_logger_and_return_cli_args,
    get_cli_args::{
        CliArgs,
        get_cli_args,
    },
};

/// # For usage info, please see the __`README.md`__ of the repo
pub fn main() -> Result<()> {
    match get_cli_args()
        .and_then(maybe_initialize_logger_and_return_cli_args)
        .and_then(|cli_args|
            match cli_args {
                CliArgs {cmd_version: true, ..} => get_tool_version_info(),
                CliArgs {cmd_signTransaction: true, ..} => sign_ethereum_transaction(cli_args),
                _ => Err(AppError::Custom(USAGE_INFO.to_string())),
            }
        ) {
            Ok(result) => {
                println!("{}", result);
                Ok(())
            },
            Err(err) => {
                println!("{}", err);
                std::process::exit(1);
            }
        }
}
