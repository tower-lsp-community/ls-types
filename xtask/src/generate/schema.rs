//! Types adapted from [metaModel.ts].
//!
//! [metaModel.ts]: https://github.com/microsoft/language-server-protocol/blob/gh-pages/_specifications/lsp/3.18/metaModel/metaModel.ts

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::generate::target;

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaModel {
    pub meta_data: MetaData,
    pub requests: Vec<Request>,
    pub notifications: Vec<Notification>,
    pub structures: Vec<Structure>,
    pub enumerations: Vec<Enumeration>,
    pub type_aliases: Vec<TypeAlias>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub method: String,
    pub type_name: String,
    pub result: Type,
    pub message_direction: MessageDirection,
    pub client_capability: Option<String>,
    pub server_capability: Option<String>,
    pub params: Option<Type>,
    pub partial_result: Option<Type>,
    pub registration_options: Option<RegistrationOptions>,
    pub documentation: Option<String>,
    pub since: Option<String>,
    pub proposed: Option<bool>,
    pub registration_method: Option<String>,
    pub error_data: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageDirection {
    ClientToServer,
    ServerToClient,
    Both,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationOptions {
    pub kind: String,
    pub name: Option<String>,
    pub items: Option<Vec<Type>>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub method: String,
    pub type_name: String,
    pub message_direction: String,
    pub server_capability: Option<String>,
    pub params: Option<Type>,
    pub documentation: Option<String>,
    pub client_capability: Option<String>,
    pub registration_options: Option<Type>,
    pub since: Option<String>,
    pub registration_method: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Structure {
    pub name: String,
    pub properties: Vec<Property>,
    #[serde(default)]
    pub extends: Vec<Type>,
    #[serde(default)]
    pub mixins: Vec<Type>,
    pub documentation: Option<String>,
    pub since: Option<String>,
    pub since_tags: Option<Vec<String>>,
    pub proposed: Option<bool>,
    pub deprecated: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: Type,
    pub documentation: Option<String>,
    pub optional: Option<bool>,
    pub since: Option<String>,
    pub since_tags: Option<Vec<String>>,
    pub proposed: Option<bool>,
    pub deprecated: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Type {
    Base { name: BaseType },
    Reference { name: String },
    Array { element: Box<Type> },
    Map { key: Box<Type>, value: Box<Type> },
    Or { items: Vec<Type> },
    Tuple { items: Vec<Type> },

    Literal { value: StructureLiteral },
    StringLiteral { value: String },
    IntegerLiteral { value: i64 },
    BooleanLiteral { value: bool },
}

impl Type {
    pub fn into_reference(self) -> Option<target::TypeRef> {
        match self {
            Self::Reference { name } => Some(target::TypeRef::new(name)),
            Self::Base { name } => match name {
                BaseType::Uri => Some(target::TypeRef::new("Uri")),
                BaseType::DocumentUri => Some(target::TypeRef::new("DocumentUri")),
                BaseType::Integer => Some(target::TypeRef::new("i64")),
                BaseType::Uinteger => Some(target::TypeRef::new("u32")),
                BaseType::String => Some(target::TypeRef::new("String")),
                BaseType::Boolean => Some(target::TypeRef::new("bool")),
                _ => None,
            },
            Self::Array { element } => element
                .into_reference()
                .map(|inner| target::TypeRef::new_generics("Vec", &[inner])),
            Self::Tuple { items } => {
                let items = items
                    .iter()
                    .map(|item| item.clone().into_reference().unwrap())
                    .collect::<Vec<_>>();
                Some(target::TypeRef::new_tuple(&items))
            }
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BaseType {
    #[serde(rename = "URI")]
    Uri,
    #[serde(rename = "DocumentUri")]
    DocumentUri,
    Integer,
    Uinteger,
    // Exact range specified in the docs
    Decimal,
    #[serde(rename = "RegExp")]
    RegExp,
    String,
    Boolean,
    Null,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enumeration {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: EnumerationType,
    pub values: Vec<EnumerationEntry>,
    #[serde(default)]
    pub supports_custom_values: bool,
    pub documentation: Option<String>,
    pub since: Option<String>,
    #[serde(default)]
    pub proposed: bool,
    pub deprecated: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum EnumerationType {
    Base { name: EnumerationTypeKind },
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EnumerationTypeKind {
    String,
    Integer,
    Uinteger,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumerationEntry {
    pub name: String,
    pub value: Value,
    pub documentation: Option<String>,
    pub since: Option<String>,
    #[serde(default)]
    pub proposed: bool,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeAlias {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: Type,
    pub documentation: Option<String>,
    pub since: Option<String>,
    #[serde(default)]
    pub proposed: bool,
    pub deprecated: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureLiteral {
    pub properties: Vec<Property>,
    pub documentation: Option<String>,
    pub since: Option<String>,
    pub since_tags: Option<Vec<String>>,
    #[serde(default)]
    pub proposed: bool,
    pub deprecated: Option<String>,
}
