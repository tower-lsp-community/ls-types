//! Language Server Protocol (LSP) and Language Server Index Format (LSIF) types.
//!
//! Based on <https://microsoft.github.io/language-server-protocol/specification>
#![allow(clippy::too_long_first_doc_paragraph)] // Docs mirrors LSP API Docs
mod macros;

pub mod error_codes;
pub mod notification;
pub mod request;
mod uri;

pub use crate::uri::Uri;

pub mod lsif;
pub mod lsp;

pub mod prelude {
    pub use crate::Uri;
    pub use crate::lsif::*;
    pub use crate::lsp::*;
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    pub fn test_serialization<SER>(ms: &SER, expected: &str)
    where
        SER: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
    {
        let json_str = serde_json::to_string(ms).unwrap();
        assert_eq!(&json_str, expected);
        let deserialized: SER = serde_json::from_str(&json_str).unwrap();
        assert_eq!(&deserialized, ms);
    }

    pub fn test_deserialization<T>(json: &str, expected: &T)
    where
        T: for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
    {
        let value = serde_json::from_str::<T>(json).unwrap();
        assert_eq!(&value, expected);
    }
}
