#![feature(try_trait)]
#![allow(clippy::match_bool)]
#![allow(clippy::too_many_arguments)]

extern crate docopt;
extern crate simplelog;
extern crate secp256k1;
extern crate tiny_keccak;
extern crate ethereum_types;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

mod lib;
mod test_utils;
mod interactive_cli_lib;

use crate::{
    interactive_cli_lib::run_interactive_cli,
    lib::{
        types::Result,
        sign_ethereum_transaction::sign_ethereum_transaction,
        get_tool_version_info::get_tool_version_info,
        initialize_logger::maybe_initialize_logger_and_return_cli_args,
        get_cli_args::{
            CliArgs,
            get_cli_args,
        },
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
                _ => run_interactive_cli(cli_args.flag_keyfile),
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
