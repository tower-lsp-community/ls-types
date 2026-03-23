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

    pub default_derives: Vec<String>,

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
        let mut null_case = false;

        let mut refs = Vec::default();
        for item in items {
            let Some(type_ref) = item.clone().into_reference() else {
                null_case = true;
                continue;
            };
            refs.push(type_ref.to_string());
        }

        let key = refs.join("|");
        let mut anon_mapping_ref = match self.anon_mappings.get(key.as_str()) {
            Some(MappingOption::Ref { r#ref }) => TypeRef::new(r#ref.clone()),
            Some(MappingOption::Enum { name, .. }) => TypeRef::new(name.clone()),
            None => return Err(Some(key)),
        };

        if null_case {
            anon_mapping_ref = TypeRef::new_generics("Option", &[anon_mapping_ref]);
        }

        Ok(anon_mapping_ref)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MappingOption {
    Ref { r#ref: String },
    Enum { name: String, variants: Vec<String> },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CodegenOption {
    Generate { generate: bool },
    Checksum { hash: String },
}
