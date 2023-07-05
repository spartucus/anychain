use crate::{BitcoinFormat, BitcoinNetwork, Prefix};
use anychain_core::no_std::*;
use anychain_core::{AddressError, Network, NetworkError};

use core::{fmt, str::FromStr};
use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct DogecoinTestnet;

impl Network for DogecoinTestnet {
    const NAME: &'static str = "dogecoin testnet";
}

pub static mut LOOP: u8 = 0;

impl BitcoinNetwork for DogecoinTestnet {
    /// Returns the address prefix of the given network.
    fn to_address_prefix(format: BitcoinFormat) -> Result<Prefix, AddressError> {
        match format {
            BitcoinFormat::P2PKH => Ok(Prefix::Version(0x71)),
            BitcoinFormat::P2WSH => Ok(Prefix::Version(0x00)),
            BitcoinFormat::P2SH_P2WPKH => Ok(Prefix::Version(0xc4)),
            f => Err(AddressError::Message(format!(
                "{} does not support address format {}",
                Self::NAME,
                f,
            ))),
        }
    }

    /// Returns the network of the given address prefix.
    fn from_address_prefix(prefix: Prefix) -> Result<Self, AddressError> {
        match prefix {
            Prefix::Version(version) => match version {
                0x71 | 0xc4 => Ok(Self),
                _ => Err(AddressError::Message(format!(
                    "Invalid version byte {:#0x} for network {}",
                    version,
                    Self::NAME,
                ))),
            },
            _ => Err(AddressError::Message(format!(
                "{} does not support address format Bech32 or CashAddr",
                Self::NAME,
            ))),
        }
    }
}

impl FromStr for DogecoinTestnet {
    type Err = NetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Self::NAME => Ok(Self),
            _ => Err(NetworkError::InvalidNetwork(s.into())),
        }
    }
}

impl fmt::Display for DogecoinTestnet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
