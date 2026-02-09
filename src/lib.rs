//! Language Server Protocol (LSP) and Language Server Index Format (LSIF) types.
//!
//! Based on <https://microsoft.github.io/language-server-protocol/specification>

use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize, de, de::Error};
use serde_json::Value;

mod generated;
