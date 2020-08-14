use std::fmt;
use crate::lib::{
    types::{
        Byte,
        Result,
    },
    errors::AppError,
};

#[derive(Clone, Copy)]
pub enum EthereumChainId {
    Kovan,
    Goerli,
    Mainnet,
    Rinkeby,
    Ropsten,
}

impl EthereumChainId {
    pub fn from_int(int: &usize) -> Result<Self> {
        match int {
            1 => Ok(EthereumChainId::Mainnet),
            3 => Ok(EthereumChainId::Ropsten),
            4 => Ok(EthereumChainId::Rinkeby),
            5 => Ok(EthereumChainId::Goerli),
            42 => Ok(EthereumChainId::Kovan),
            _ => Err(AppError::Custom(format!("✘ Unrecognised chain id: '{}'!", int)))
        }
    }

    pub fn to_byte(&self) -> Byte {
        self.to_int() as u8
    }

    pub fn to_int(&self) -> usize {
        match self {
            EthereumChainId::Mainnet => 1,
            EthereumChainId::Ropsten => 3,
            EthereumChainId::Rinkeby => 4,
            EthereumChainId::Goerli => 5,
            EthereumChainId::Kovan => 42,
        }
    }

    pub fn from_str(chain_str: &str) -> Result<Self> {
        match chain_str {
            "Kovan" | "kovan" => Ok(EthereumChainId::Kovan),
            "Goerli" | "goerli" => Ok(EthereumChainId::Goerli),
            "Mainnet" | "mainnet" => Ok(EthereumChainId::Mainnet),
            "Ropsten" | "ropsten" => Ok(EthereumChainId::Ropsten),
            "Rinkeby" | "rinkeby" => Ok(EthereumChainId::Rinkeby),
            _ => Err(AppError::Custom(format!("✘ Unrecognised chain: '{}'!", chain_str)))
        }
    }
}

impl fmt::Display for EthereumChainId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EthereumChainId::Mainnet => write!(f, "Mainnet"),
            EthereumChainId::Kovan => write!(f, "Kovan Testnet"),
            EthereumChainId::Goerli => write!(f, "Goerli Testnet"),
            EthereumChainId::Ropsten => write!(f, "Ropsten Testnet"),
            EthereumChainId::Rinkeby => write!(f, "Rinkeby Testnet"),
        }
    }
}
