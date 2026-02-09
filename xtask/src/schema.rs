//! Types adapted from [metaModel.ts].
//!
//! [metaModel.ts]: https://github.com/microsoft/language-server-protocol/blob/gh-pages/_specifications/lsp/3.18/metaModel/metaModel.ts

use serde::{Deserialize, Serialize};
use serde_json::Value;
use smol_str::SmolStr;

use crate::target;

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
    pub version: SmolStr,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub method: SmolStr,
    pub type_name: SmolStr,
    pub result: Result,
    pub message_direction: MessageDirection,
    pub client_capability: Option<SmolStr>,
    pub server_capability: Option<SmolStr>,
    pub params: Option<Type>,
    pub partial_result: Option<Type>,
    pub registration_options: Option<RegistrationOptions>,
    pub documentation: Option<SmolStr>,
    pub since: Option<SmolStr>,
    pub proposed: Option<bool>,
    pub registration_method: Option<SmolStr>,
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
pub struct Result {
    pub kind: SmolStr,
    #[serde(default)]
    pub items: Vec<Item>,
    pub element: Option<Type>,
    pub name: Option<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub kind: SmolStr,
    pub name: Option<SmolStr>,
    pub element: Option<Element>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    pub kind: SmolStr,
    pub name: Option<SmolStr>,
    pub items: Option<Vec<Type>>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationOptions {
    pub kind: SmolStr,
    pub name: Option<SmolStr>,
    pub items: Option<Vec<Type>>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub method: SmolStr,
    pub type_name: SmolStr,
    pub message_direction: SmolStr,
    pub server_capability: Option<SmolStr>,
    pub params: Option<Type>,
    pub documentation: Option<SmolStr>,
    pub client_capability: Option<SmolStr>,
    pub registration_options: Option<Type>,
    pub since: Option<SmolStr>,
    pub registration_method: Option<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Structure {
    pub name: SmolStr,
    pub properties: Vec<Property>,
    #[serde(default)]
    pub extends: Vec<Type>,
    #[serde(default)]
    pub mixins: Vec<Type>,
    pub documentation: Option<SmolStr>,
    pub since: Option<SmolStr>,
    pub since_tags: Option<Vec<SmolStr>>,
    pub proposed: Option<bool>,
    pub deprecated: Option<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub name: SmolStr,
    #[serde(rename = "type")]
    pub type_: Type,
    pub documentation: Option<SmolStr>,
    pub optional: Option<bool>,
    pub since: Option<SmolStr>,
    pub since_tags: Option<Vec<SmolStr>>,
    pub proposed: Option<bool>,
    pub deprecated: Option<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Type {
    Base { name: BaseType },
    Reference { name: SmolStr },
    Array { element: Box<Type> },
    Map { key: Box<Type>, value: Box<Type> },
    Or { items: Vec<Type> },
    Tuple { items: Vec<Type> },

    Literal { value: StructureLiteral },
    StringLiteral { value: SmolStr },
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
            _ => None,
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
    pub name: SmolStr,
    #[serde(rename = "type")]
    pub type_: EnumerationType,
    pub values: Vec<EnumerationEntry>,
    #[serde(default)]
    pub supports_custom_values: bool,
    pub documentation: Option<SmolStr>,
    pub since: Option<SmolStr>,
    #[serde(default)]
    pub proposed: bool,
    pub deprecated: Option<SmolStr>,
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
    pub name: SmolStr,
    pub value: Value,
    pub documentation: Option<SmolStr>,
    pub since: Option<SmolStr>,
    #[serde(default)]
    pub proposed: bool,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeAlias {
    pub name: SmolStr,
    #[serde(rename = "type")]
    pub type_: Type,
    pub documentation: Option<SmolStr>,
    pub since: Option<SmolStr>,
    #[serde(default)]
    pub proposed: bool,
    pub deprecated: Option<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureLiteral {
    pub properties: Vec<Property>,
    pub documentation: Option<SmolStr>,
    pub since: Option<SmolStr>,
    pub since_tags: Option<Vec<SmolStr>>,
    #[serde(default)]
    pub proposed: bool,
    pub deprecated: Option<SmolStr>,
}
