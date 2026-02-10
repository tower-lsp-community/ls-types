use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use smol_str::{SmolStr, ToSmolStr};

use crate::{
    schema,
    target::{self, TypeRef},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Config {
    pub version: String,

    pub anon_mappings: BTreeMap<SmolStr, SmolStr>,
    pub structs: BTreeMap<SmolStr, CodegenOption>,
    pub enums: BTreeMap<SmolStr, CodegenOption>,
}

impl Config {
    /// Return the anonymous mapping if items are a valid reference collection.
    ///
    /// If it fails, it will return the key if the items form a valid collection.
    pub(crate) fn lookup_anon(
        &self,
        items: &[schema::Type],
    ) -> Result<target::TypeRef, Option<String>> {
        let refs = items
            .iter()
            .map(|item| item.clone().into_reference())
            .collect::<Option<Vec<TypeRef>>>()
            .ok_or(None)?;
        let key = refs
            .iter()
            .map(|ref_| ref_.as_str().to_smolstr())
            .collect::<Vec<_>>()
            .join("|");

        match self.anon_mappings.get(key.as_str()) {
            Some(ref_) => Ok(TypeRef::new(ref_.clone())),
            None => Err(Some(key)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CodegenOption {
    Generate(bool),
    Checksum(SmolStr),
}
