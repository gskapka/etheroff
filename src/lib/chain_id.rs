use crate::lib::{
    types::Result,
    errors::AppError,
};

pub enum EthereumChainId {
    Mainet,
    Rinkeby,
    Ropsten,
    Kovan,
    Goerli,
}

impl EthereumChainId {
    pub fn default() -> Self {
        EthereumChainId::Mainet
    }

    pub fn from_int(int: &usize) -> Result<Self> {
        match int {
            1 => Ok(EthereumChainId::Mainet),
            3 => Ok(EthereumChainId::Ropsten),
            4 => Ok(EthereumChainId::Rinkeby),
            5 => Ok(EthereumChainId::Goerli),
            42 => Ok(EthereumChainId::Kovan),
            _ => Err(AppError::Custom(format!("✘ Unrecognised chain id: '{}'!", int)))
        }
    }

    pub fn to_int(&self) -> usize {
        match self {
            EthereumChainId::Mainet => 1,
            EthereumChainId::Ropsten => 3,
            EthereumChainId::Rinkeby => 4,
            EthereumChainId::Goerli => 5,
            EthereumChainId::Kovan => 42,
        }
    }
}
