use std::{
    collections::{BTreeSet, HashMap},
    hash::{Hash, Hasher},
    io,
};

use eyre::{Context, bail};
use serde::{Deserialize, de::DeserializeOwned};
use smol_str::{SmolStr, ToSmolStr};

use crate::{
    config::{CodegenOption, Config},
    schema::{self, EnumerationEntry, StructureLiteral},
    target,
};

pub fn translate_schema(
    meta_model: &schema::MetaModel,
    config: &Config,
) -> eyre::Result<Vec<target::Item>> {
    let mut t = Translator {
        config: config.clone(),
        structs_missing: Default::default(),
        enums_missing: Default::default(),
        anon_missing: Default::default(),
    };
    meta_model.translate(&mut t)
}

struct Translator {
    config: Config,

    structs_missing: BTreeSet<SmolStr>,
    enums_missing: BTreeSet<SmolStr>,
    anon_missing: BTreeSet<SmolStr>,
}

trait TranslateSchema {
    type Output;
    fn translate(&self, t: &mut Translator) -> eyre::Result<Self::Output>;
}

impl TranslateSchema for schema::MetaModel {
    type Output = Vec<target::Item>;
    fn translate(&self, t: &mut Translator) -> eyre::Result<Self::Output> {
        let Self {
            meta_data,
            requests,
            notifications,
            structures,
            enumerations,
            type_aliases,
        } = self;

        let mut items = Vec::new();
        for structure in structures {
            match t.config.structs.remove(&structure.name) {
                Some(CodegenOption::Generate(true | false)) => {
                    // FIXME(wiro): use false to not actually generate the item, makes it mixin only
                    let struct_ = structure
                        .translate(t)
                        .wrap_err_with(|| format!("translating structure {}", structure.name))?;
                    items.push(target::Item::Struct(struct_));
                }
                Some(CodegenOption::Checksum(cksum)) => {
                    let hash = {
                        let mut hasher = fasthash::xx::Hasher32::default();
                        structure.hash(&mut hasher);
                        format!("{:08x}", hasher.finish())
                    };
                    if hash != *cksum {
                        eprintln!(
                            "Hash mismatch for structure {}, expected {:?}, got {:?}",
                            structure.name, cksum, hash
                        );
                    }
                }
                None => {
                    t.structs_missing.insert(structure.name.clone());
                    continue;
                }
            };
        }
        if !t.structs_missing.is_empty() {
            eprintln!("These structs are missing. Add them.\n```toml\n[structs]");
            for missing in &t.structs_missing {
                eprintln!("{missing} = true");
            }
            eprintln!("```");
        }

        for enumeration in enumerations {
            match t.config.enums.remove(&enumeration.name) {
                Some(CodegenOption::Generate(true | false)) => {
                    let enum_ = enumeration.translate(t).wrap_err_with(|| {
                        format!("translating enumeration {}", enumeration.name)
                    })?;
                    items.push(target::Item::Enum(enum_));
                }
                Some(CodegenOption::Checksum(cksum)) => {
                    let hash = {
                        let mut hasher = fasthash::xx::Hasher32::default();
                        enumeration.hash(&mut hasher);
                        format!("{:08x}", hasher.finish())
                    };
                    if hash != *cksum {
                        eprintln!(
                            "Hash mismatch for enumeration {}, expected {:?}, got {:?}",
                            enumeration.name, cksum, hash
                        );
                    }
                }
                None => {
                    t.enums_missing.insert(enumeration.name.clone());
                    continue;
                }
            };
        }
        if !t.enums_missing.is_empty() {
            eprintln!("These enums are missing. Add them.\n```toml\n[enums]");
            for missing in &t.enums_missing {
                eprintln!("{missing} = true");
            }
            eprintln!("```");
        }

        if !t.anon_missing.is_empty() {
            eprintln!("These anon mappings are missing. Add them.\n```toml\n[anon-mappings]");
            for missing in &t.anon_missing {
                eprintln!("\"{missing}\" = \"todo\"");
            }
            eprintln!("```");
        }

        todo!()
    }
}

impl TranslateSchema for schema::Structure {
    type Output = target::Struct;
    fn translate(&self, t: &mut Translator) -> eyre::Result<Self::Output> {
        let Self {
            name,
            properties,
            extends,
            mixins,
            documentation,
            since,
            since_tags: _,
            proposed,
            deprecated,
        } = self;

        let extends = extends
            .iter()
            .chain(mixins)
            .map(|ty| ty.clone().into_reference().unwrap().as_str().to_smolstr())
            .collect();

        let mut fields = Vec::default();

        for schema::Property {
            name,
            type_,
            documentation,
            optional,
            since,
            since_tags,
            proposed,
            deprecated,
        } in properties
        {
            let Some(ty) = type_
                .translate(t)
                .wrap_err_with(|| format!("while translating property: {name}"))?
            else {
                continue;
            };

            fields.push(target::StructFields {
                name: name.clone(),
                ty,
                doc: documentation.clone(),
                since: target::Version::parse(since.as_deref())?,
                deprecated: deprecated.clone(),
            });
        }

        Ok(target::Struct {
            name: name.clone(),
            extends,
            fields,
            doc: documentation.clone(),
            since: target::Version::parse(since.as_deref())?,
            deprecated: deprecated.clone(),
        })
    }
}

impl TranslateSchema for schema::Enumeration {
    type Output = target::Enum;
    fn translate(&self, t: &mut Translator) -> eyre::Result<Self::Output> {
        let Self {
            name,
            type_,
            values,
            supports_custom_values,
            documentation,
            since,
            proposed,
            deprecated,
        } = self;

        let schema::EnumerationType::Base { name: kind } = type_;
        match kind {
            schema::EnumerationTypeKind::String => { /* default, nothing to do */ }
            schema::EnumerationTypeKind::Integer => {
                bail!("translate int enums")
            }
            schema::EnumerationTypeKind::Uinteger => {
                bail!("translate uint enums")
            }
        }

        let mut variants = Vec::new();

        for schema::EnumerationEntry {
            name,
            value,
            documentation,
            since,
            proposed,
        } in values
        {
            variants.push(target::EnumVariants {
                name: name.clone(),
                doc: documentation.clone(),
                since: target::Version::parse(since.as_deref())?,
            });
        }

        Ok(target::Enum {
            name: name.clone(),
            variants,
            doc: documentation.clone(),
            since: target::Version::parse(since.as_deref())
                .unwrap_or_else(|s| panic!("invalid version {s}")),
            deprecated: deprecated.clone(),
        })
    }
}

impl TranslateSchema for schema::Type {
    type Output = Option<target::TypeRef>;
    fn translate(&self, t: &mut Translator) -> eyre::Result<Self::Output> {
        let ty = match self {
            schema::Type::Base { name } => match name {
                schema::BaseType::Uri => target::TypeRef::new("crate::Uri"),

                // TODO: have a separate type for document uri
                schema::BaseType::DocumentUri => target::TypeRef::new("crate::DocumentUri"),
                schema::BaseType::Integer => target::TypeRef::new("i64"),
                schema::BaseType::Uinteger => target::TypeRef::new("u32"),
                schema::BaseType::Decimal => target::TypeRef::new("f32"),
                schema::BaseType::RegExp => panic!("present in the spec but not in the metaModel"),
                schema::BaseType::String => target::TypeRef::new("String"),
                schema::BaseType::Boolean => target::TypeRef::new("bool"),
                schema::BaseType::Null => todo!(),
            },
            schema::Type::Array { element } => {
                let Some(element) = element.translate(t)? else {
                    return Ok(None);
                };
                target::TypeRef::new_generics("Vec", &[element])
            }
            schema::Type::Reference { name } => target::TypeRef::new(name.clone()),
            schema::Type::Map { key, value } => {
                let Some(key) = key.translate(t)? else {
                    return Ok(None);
                };
                let Some(value) = value.translate(t)? else {
                    return Ok(None);
                };
                target::TypeRef::new_generics("std::collections::HashMap", &[key, value])
            }
            schema::Type::Or { items } => {
                // FIXME(wiro): do not keep empty objects `{}`?
                let mut items = items.clone();
                items.retain(|item| {
                    !matches!(
                        item,
                        schema::Type::Literal {
                            value: StructureLiteral { properties, .. }
                        }
                        if properties.is_empty()
                    )
                });
                if let [ty] = items.as_slice() {
                    return ty.translate(t);
                }
                // ENDFIXME

                if let [
                    ty,
                    schema::Type::Base {
                        name: schema::BaseType::Null,
                    },
                ] = items.as_slice()
                {
                    let Some(inner) = ty.translate(t)? else {
                        return Ok(None);
                    };
                    target::TypeRef::new_generics("Option", &[inner])
                } else {
                    match t.config.lookup_anon(&items) {
                        Ok(ref_) => ref_,
                        Err(None) => bail!("invalid collection of items for an enum: {items:#?}"),
                        Err(Some(key)) => {
                            t.anon_missing.insert(key.to_smolstr());
                            target::TypeRef::new("todo!()")
                        }
                    }
                }
            }
            schema::Type::Tuple { items } => bail!("translate tuples: {items:?}"),
            schema::Type::Literal { value } => bail!("translate literal type {value:?}"),
            schema::Type::StringLiteral { value } => {
                // TODO
                // bail!("translate string literal type {value:?}")
                return Ok(None);
            }
            schema::Type::IntegerLiteral { value } => {
                bail!("translate string integer type {value:?}")
            }
            schema::Type::BooleanLiteral { value } => {
                bail!("translate string boolean type {value:?}")
            }
        };
        Ok(Some(ty))
    }
}
