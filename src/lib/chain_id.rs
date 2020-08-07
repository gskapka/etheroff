use std::fmt;
use crate::lib::{
    types::{
        Byte,
        Result,
    },
    errors::AppError,
};

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
