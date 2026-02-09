//! Language Server Protocol (LSP) and Language Server Index Format (LSIF) types.
//!
//! Based on <https://microsoft.github.io/language-server-protocol/specification>

use std::{collections::HashMap, fmt::Debug};

use serde::{
    Deserialize, Serialize,
    de::{self, DeserializeOwned, Error},
};
use serde_json::Value;

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
