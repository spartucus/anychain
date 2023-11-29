use crate::no_std::String;
use crate::AddressError;
use crate::AmountError;
use crate::FormatError;
use crate::PublicKeyError;
use crate::TransactionError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Runtime Error:{0}")]
    RuntimeError(String),

    #[error("Invalid Address: {0}")]
    InvalidAddress(#[from] AddressError),

    #[error("Invalid Transaction: {0:}")]
    InvalidTransaction(#[from] TransactionError),

    #[error("Invalid Amount: {0:}")]
    InvalidAmount(#[from] AmountError),

    #[error("Invalid PublickKey: {0:}")]
    InvalidPublickKey(#[from] PublicKeyError),

    #[error("Invalid Format: {0:}")]
    InvalidFormat(#[from] FormatError),

    // #[cfg(feature = "std")]
    // #[error("io error: {0:}")]
    // Io(#[from] std::io::Error),
    //
    // #[cfg(not(feature = "std"))]
    // #[error("io error: {0:}")]
    // Io(#[from] crate::no_std::io::Error),
    #[cfg(feature = "std")]
    #[error("fmt error: {0:}")]
    Fmt(#[from] ::std::fmt::Error),

    #[cfg(not(feature = "std"))]
    #[error("fmt error: {0:}")]
    Fmt(#[from] ::core::fmt::Error),

    // #[error("fromHex error: {0:}")]
    // FromHex(#[from] ::hex::FromHexError),
    #[cfg(feature = "std")]
    #[error("parsing error: {0:}")]
    ParseInt(#[from] ::std::num::ParseIntError),

    #[cfg(not(feature = "std"))]
    #[error("parsing error: {0:}")]
    ParseInt(#[from] ::core::num::ParseIntError),
    // #[error("secp265k1 error: {0:}")]
    // Secp256k1Error(#[from] ::libsecp256k1::Error),
}
