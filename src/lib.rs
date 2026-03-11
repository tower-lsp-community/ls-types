//! Language Server Protocol (LSP) and Language Server Index Format (LSIF) types.
//!
//! *[Specification](https://microsoft.github.io/language-server-protocol/specification)*

use serde::{Serialize, de::DeserializeOwned};

#[allow(unused_imports)]
mod generated;
mod manual;

pub trait Notification {
    type Params: DeserializeOwned + Serialize + Send + Sync + 'static;
    const METHOD: &'static str;
}

pub trait Request {
    type Params: DeserializeOwned + Serialize + Send + Sync + 'static;
    type Result: DeserializeOwned + Serialize + Send + Sync + 'static;
    const METHOD: &'static str;
}

pub type Uri = fluent_uri::Uri<String>;
pub type DocumentUri = Uri;

#[deprecated]
type Todo = ();

// Export generated types
pub use generated::*;
pub use manual::*;

/// Re-export specification types for glob import
pub mod prelude {
    pub use crate::generated::*;
    pub use crate::manual::*;
}
