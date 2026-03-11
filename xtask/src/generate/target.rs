use std::{fmt, ops, str::FromStr};

use eyre::bail;

#[derive(Debug, Clone, Copy)]
pub enum Version {
    Unknown,
    V3_2_0,
    V3_6_0,
    V3_8_0,
    V3_10_0,
    V3_12_0,
    V3_13_0,
    V3_14_0,
    V3_15_0,
    V3_16_0,
    V3_17_0,
    V3_18_0,
}

impl FromStr for Version {
    type Err = eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match {
            // FIXME: upstream inconsistencies to the specification
            let s = s.strip_prefix("version ").unwrap_or(s);
            let s = s
                .split_once(' ')
                .map(|(version, _details)| version)
                .unwrap_or(s);
            s.strip_suffix('.').unwrap_or(s)
        } {
            "3.2.0" => Ok(Self::V3_2_0),
            "3.6.0" => Ok(Self::V3_6_0),
            "3.8.0" => Ok(Self::V3_8_0),
            "3.10.0" => Ok(Self::V3_10_0),
            "3.12.0" => Ok(Self::V3_12_0),
            "3.13.0" => Ok(Self::V3_13_0),
            "3.14.0" => Ok(Self::V3_14_0),
            "3.15.0" => Ok(Self::V3_15_0),
            "3.16.0" => Ok(Self::V3_16_0),
            // FIXME(wiro): fix version in spec
            "3.16" => Ok(Self::V3_17_0),
            "3.17.0" => Ok(Self::V3_17_0),
            // FIXME(wiro): fix version in spec
            "3.17" => Ok(Self::V3_17_0),
            "3.18.0" => Ok(Self::V3_18_0),
            version => bail!("invalid version {version}"),
        }
    }
}

impl TryFrom<&Option<String>> for Version {
    type Error = eyre::Report;
    fn try_from(value: &Option<String>) -> Result<Self, Self::Error> {
        if let Some(value) = value {
            value.parse()
        } else {
            Ok(Version::Unknown)
        }
    }
}

impl Version {
    pub fn as_str(&self) -> Option<&'static str> {
        match self {
            Self::Unknown => None,
            Self::V3_2_0 => Some("3.2.0"),
            Self::V3_6_0 => Some("3.6.0"),
            Self::V3_8_0 => Some("3.8.0"),
            Self::V3_10_0 => Some("3.10.0"),
            Self::V3_12_0 => Some("3.12.0"),
            Self::V3_13_0 => Some("3.13.0"),
            Self::V3_14_0 => Some("3.14.0"),
            Self::V3_15_0 => Some("3.15.0"),
            Self::V3_16_0 => Some("3.16.0"),
            Self::V3_17_0 => Some("3.17.0"),
            Self::V3_18_0 => Some("3.18.0"),
        }
    }
}

#[derive(Debug)]
pub struct MetaModel {
    pub imports: Vec<String>,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    Struct(Struct),
    Enum(Enum),
    TypeAlias(TypeAlias),

    TraitImpl(TraitImpl),
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    // derives: Vec<Derive>,
    // methods: Vec<...>,
    // from/into impl
    pub extends: Vec<String>,
    pub fields: Vec<StructFields>,
    pub doc: Documentation,
    pub deprecated: Option<String>,
    pub since: Version,
}

#[derive(Debug, Clone)]
pub struct StructFields {
    pub name: String,
    pub ty: TypeRef,
    pub doc: Documentation,
    pub since: Version,
    pub deprecated: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TypeRef(String);

impl TypeRef {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub(crate) fn new_generics(name: impl Into<String>, generics: &[TypeRef]) -> Self {
        let smol_str = format!(
            "{}<{}>",
            name.into(),
            generics
                .iter()
                .map(|t| t.0.clone())
                .collect::<Vec<_>>()
                .join(",")
        )
        .into();
        Self(smol_str)
    }

    pub(crate) fn new_tuple(elements: &[TypeRef]) -> Self {
        let tuple = format!(
            "({})",
            elements
                .iter()
                .map(|t| t.0.clone())
                .collect::<Vec<_>>()
                .join(",")
        );
        Self(tuple.into())
    }
}

impl ops::Deref for TypeRef {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for TypeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    // derives: Vec<Derive>,
    // methods: Vec<...>,
    // from/into impl
    pub variants: Vec<EnumVariant>,
    pub doc: Documentation,
    pub deprecated: Option<String>,
    pub since: Version,
}

#[derive(Debug)]
pub struct EnumVariant {
    pub name: String,
    pub value: EnumVariantKind,
    pub doc: Documentation,
    pub since: Version,
}

#[derive(Debug)]
pub enum EnumVariantKind {
    Unit,
    Tuple(Vec<TypeRef>),
}

// TODO: codegen as wrapper struct?
#[derive(Debug)]
pub struct TypeAlias {
    pub name: String,
    // derives: Vec<Derive>,
    // methods: Vec<...>,
    // from/into impl
    pub ty: TypeRef,
    pub doc: Documentation,
    pub deprecated: Option<String>,
    pub since: Version,
}

#[derive(Debug)]
pub struct TraitImpl {
    pub interface: String,
    pub implementor: String,
    pub assoc_types: Vec<(String, TypeRef)>,
    pub assoc_const: Vec<(String, TypeRef, String)>,
}

#[derive(Debug, Clone, Default)]
pub struct Documentation(pub String);

impl Documentation {
    fn parse(doc: &str) -> Self {
        Self(doc.into())
    }
}

impl From<&Option<String>> for Documentation {
    fn from(value: &Option<String>) -> Self {
        value
            .as_ref()
            .map(|s| Documentation::parse(&s))
            .unwrap_or_default()
    }
}
