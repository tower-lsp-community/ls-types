use std::{convert::Infallible, fmt};

use eyre::bail;
use smol_str::SmolStr;
use thin_vec::ThinVec;

#[derive(Debug)]
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

impl Version {
    pub fn parse(s: Option<&str>) -> eyre::Result<Self> {
        match s.map(|s| {
            let s = s.strip_prefix("version ").unwrap_or(s);
            let s = s
                .split_once(' ')
                .map(|(version, details)| version)
                .unwrap_or(s);
            s.strip_suffix('.').unwrap_or(s)
        }) {
            None => Ok(Self::Unknown),
            Some("3.2.0") => Ok(Self::V3_2_0),
            Some("3.6.0") => Ok(Self::V3_6_0),
            Some("3.8.0") => Ok(Self::V3_8_0),
            Some("3.10.0") => Ok(Self::V3_10_0),
            Some("3.12.0") => Ok(Self::V3_12_0),
            Some("3.13.0") => Ok(Self::V3_13_0),
            Some("3.14.0") => Ok(Self::V3_14_0),
            Some("3.15.0") => Ok(Self::V3_15_0),
            Some("3.16.0") => Ok(Self::V3_16_0),
            // FIXME(wiro): fix version in spec
            Some("3.16") => Ok(Self::V3_17_0),
            Some("3.17.0") => Ok(Self::V3_17_0),
            // FIXME(wiro): fix version in spec
            Some("3.17") => Ok(Self::V3_17_0),
            Some("3.18.0") => Ok(Self::V3_18_0),
            Some(version) => bail!("invalid version {version}"),
        }
    }
}

#[derive(Debug)]
pub enum Item {
    Struct(Struct),
    Enum(Enum),
    TraitImpl(TraitImpl),
}

#[derive(Debug)]
pub struct Struct {
    pub name: SmolStr,
    // derives: Vec<Derive>,
    // methods: Vec<...>,
    // from/into impl
    pub extends: Vec<SmolStr>,
    pub fields: Vec<StructFields>,
    pub doc: Option<SmolStr>,
    pub deprecated: Option<SmolStr>,
    pub since: Version,
}

#[derive(Debug)]
pub struct StructFields {
    pub name: SmolStr,
    pub ty: TypeRef,
    pub doc: Option<SmolStr>,
    pub since: Version,
    pub deprecated: Option<SmolStr>,
}

#[derive(Debug)]
pub struct TypeRef(SmolStr);

impl TypeRef {
    pub(crate) fn new(name: impl Into<SmolStr>) -> Self {
        Self(name.into())
    }

    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn new_generics(name: impl Into<SmolStr>, generics: &[TypeRef]) -> Self {
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

#[derive(Debug)]
pub struct Enum {
    pub name: SmolStr,
    // derives: Vec<Derive>,
    // methods: Vec<...>,
    // from/into impl
    pub variants: Vec<EnumVariants>,
    pub doc: Option<SmolStr>,
    pub deprecated: Option<SmolStr>,
    pub since: Version,
}

#[derive(Debug)]
pub struct EnumVariants {
    pub name: SmolStr,
    // pub value: Value,
    pub doc: Option<SmolStr>,
    pub since: Version,
}

#[derive(Debug)]
pub struct TraitImpl {
    pub interface: SmolStr,
    pub implementor: SmolStr,
    pub assoc_types: Vec<(SmolStr, TypeRef)>,
    pub assoc_const: Vec<(SmolStr, String)>,
}
