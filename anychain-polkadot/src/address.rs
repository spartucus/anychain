use std::{fmt::Display, marker::PhantomData, str::FromStr};

use anychain_core::{hex, libsecp256k1, Address, AddressError, PublicKey, TransactionError};

use crate::{PolkadotFormat, PolkadotNetwork, PolkadotPublicKey, PolkadotSecretKey, PublicKeyContent};
use base58::{FromBase58, ToBase58};
use sp_core::hashing::{blake2_256, blake2_512};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct PolkadotAddress<N: PolkadotNetwork> {
    addr: String,
    _network: PhantomData<N>,
}

impl<N: PolkadotNetwork> Address for PolkadotAddress<N> {
    type SecretKey = PolkadotSecretKey;
    type PublicKey = PolkadotPublicKey<N>;
    type Format = PolkadotFormat;

    fn from_secret_key(
        secret_key: &Self::SecretKey,
        format: &Self::Format,
    ) -> Result<Self, anychain_core::AddressError> {
        Self::PublicKey::from_secret_key(secret_key).to_address(format)
    }

    fn from_public_key(
        public_key: &Self::PublicKey,
        _format: &Self::Format,
    ) -> Result<Self, anychain_core::AddressError> {
        Self::from_payload(&hex::encode(&public_key.address_payload()))
    }
}

impl<N: PolkadotNetwork> PolkadotAddress<N> {
    pub fn from_payload(payload: &str) -> Result<Self, AddressError> {
        let payload = hex::decode(payload).unwrap();
        let payload = [vec![N::version()], payload].concat();

        let ss_prefix = vec![0x53u8, 0x53, 0x35, 0x38, 0x50, 0x52, 0x45];

        let checksum = blake2_512(&[ss_prefix, payload.clone()].concat()).to_vec();
        let addr = [payload, checksum[..2].to_vec()].concat().to_base58();

        Ok(PolkadotAddress {
            addr,
            _network: PhantomData::<N>,
        })
    }

    pub fn to_payload(&self) -> Result<Vec<u8>, AddressError> {
        let bin = self.addr.as_str().from_base58()?;
        Ok(bin[1..33].to_vec())
    }
}

impl<N: PolkadotNetwork> FromStr for PolkadotAddress<N> {
    type Err = TransactionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.from_base58()?;
        if N::version() != bytes[0] {
            return Err(TransactionError::Message(format!(
                "Invalid version byte {} for polkadot network {}",
                bytes[0],
                N::NAME,
            )));
        }
        let checksum_provided = bytes[33..].to_vec();
        let ss_prefix = vec![0x53u8, 0x53, 0x35, 0x38, 0x50, 0x52, 0x45];
        let checksum_expected =
            blake2_512(&[ss_prefix, bytes[..33].to_vec()].concat())[..2].to_vec();
        if checksum_expected != checksum_provided {
            return Err(TransactionError::Message(format!(
                "Invalid {} address",
                N::NAME
            )));
        }
        Ok(PolkadotAddress {
            addr: s.to_string(),
            _network: PhantomData,
        })
    }
}

impl<N: PolkadotNetwork> Display for PolkadotAddress<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.addr)
    }
}

#[cfg(test)]
mod tests {
    use ed25519_dalek_fiat::SecretKey;
    use crate::{Polkadot, PolkadotAddress, PolkadotFormat, Substrate, PolkadotSecretKey};
    use anychain_core::Address;

    #[test]
    fn test_address() {
        let sk = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        ];
        let sk = SecretKey::from_bytes(&sk).unwrap();
        let sk = PolkadotSecretKey::Ed25519(sk);

        let address =
            PolkadotAddress::<Substrate>::from_secret_key(&sk, &PolkadotFormat::Standard).unwrap();

        println!("address = {}", address);
    }

    #[test]
    fn test_address_2() {
        let hash = "0c2f3c6dabb4a0600eccae87aeaa39242042f9a576aa8dca01e1b419cf17d7a2";
        let address = PolkadotAddress::<Polkadot>::from_payload(hash).unwrap();
        println!("address = {}", address);
    }
}
