//! Mina signature structure and associated helpers

use crate::utils::FieldHelpers;
use crate::{BaseField, ScalarField};
use std::fmt;

/// Signature structure
#[derive(Clone, Eq, fmt::Debug, PartialEq)]
pub struct Signature {
    /// Base field component
    pub rx: BaseField,

    /// Scalar field component
    pub s: ScalarField,
}

impl Signature {
    /// Create a new signature
    pub fn new(rx: BaseField, s: ScalarField) -> Self {
        Self { rx, s }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let rx = BaseField::from_bytes(&bytes[..32]).unwrap();
        let s = ScalarField::from_bytes(&bytes[32..]).unwrap();
        Self::new(rx, s)
    }

    pub fn field(&self) -> String {
        let mut rx = self.rx.to_bytes();
        rx.reverse();
        hex::encode(rx)
    }

    pub fn scalar(&self) -> String {
        let mut s = self.s.to_bytes();
        s.reverse();
        hex::encode(s)
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rx_bytes = self.rx.to_bytes();
        let mut s_bytes = self.s.to_bytes();
        rx_bytes.reverse();
        s_bytes.reverse();

        write!(
            f,
            "field: {}, scalar: {}",
            hex::encode(rx_bytes),
            hex::encode(s_bytes)
        )
    }
}