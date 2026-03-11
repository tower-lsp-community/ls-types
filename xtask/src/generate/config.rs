use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::generate::{
    schema,
    target::{self, TypeRef},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Config {
    pub version: String,

    pub anon_mappings: BTreeMap<String, MappingOption>,
    pub structs: BTreeMap<String, CodegenOption>,
    pub enums: BTreeMap<String, CodegenOption>,
    pub type_aliases: BTreeMap<String, CodegenOption>,
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
            .map(|ref_| &**ref_)
            .collect::<Vec<_>>()
            .join("|");

        match self.anon_mappings.get(key.as_str()) {
            Some(MappingOption::Ref(ref_)) => Ok(TypeRef::new(ref_.clone())),
            Some(MappingOption::Enum(name, _)) => Ok(TypeRef::new(name.clone())),
            None => Err(Some(key)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MappingOption {
    Ref(String),
    Enum(String, Vec<String>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CodegenOption {
    Generate(bool),
    Checksum(String),
}
