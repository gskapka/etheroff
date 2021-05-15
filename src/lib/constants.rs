use ethereum_types::U256;

pub const ETH_NUM_DECIMALS: usize = 18;
pub const ONE_GWEI: u64 = 1_000_000_000;
pub const PRIVATE_KEY_HEX_LENGTH: usize = 64;
pub const TOOL_VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    pub static ref ONE_ETH_IN_WEI: U256 =
        U256::from_dec_str("1000000000000000000").expect("Converting 1 ETH to Wei should not fail!");
}
