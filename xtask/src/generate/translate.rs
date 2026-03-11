use std::{
    collections::{BTreeMap, BTreeSet},
    hash::{Hash, Hasher},
    str::FromStr,
};

use eyre::{Context, bail};

use crate::generate::{
    config::{CodegenOption, Config, MappingOption},
    schema,
    target::{self, TypeRef},
};

pub fn translate_schema(
    meta_model: &schema::MetaModel,
    config: &Config,
) -> eyre::Result<target::MetaModel> {
    let mut t = Translator {
        config: config.clone(),
        structs_missing: Default::default(),
        enums_missing: Default::default(),
        type_aliases_missing: Default::default(),
        anon_missing: Default::default(),
    };
    meta_model.translate(&mut t)
}

struct Translator {
    config: Config,

    structs_missing: BTreeSet<String>,
    enums_missing: BTreeSet<String>,
    type_aliases_missing: BTreeSet<String>,
    anon_missing: BTreeSet<String>,
}

trait TranslateSchema {
    type Output;
    fn translate(&self, t: &mut Translator) -> eyre::Result<Self::Output>;
}

impl TranslateSchema for schema::MetaModel {
    type Output = target::MetaModel;
    fn translate(&self, t: &mut Translator) -> eyre::Result<Self::Output> {
        let Self {
            meta_data,
            requests,
            notifications,
            structures,
            enumerations,
            type_aliases,
        } = self;

        let mut imports = Vec::new();
        let mut items = Vec::new();
        let mut mixins = BTreeMap::new();

        let schema::MetaData { version } = &meta_data;
        let version = target::Version::from_str(&version)?;

        if version.as_str().unwrap() != t.config.version {
            bail!(
                "config version mismatch: {} != {}",
                version.as_str().unwrap(),
                t.config.version
            )
        }

        for (key, anon_mapping) in &t.config.anon_mappings {
            let MappingOption::Enum(name, variants) = anon_mapping else {
                continue;
            };
            let variants_tys = key.split("|").collect::<Vec<_>>();

            if variants_tys.len() != variants.len() {
                eprintln!("variants count mismatch for anon mapping `{name}`");
                continue;
            }

            let variants = variants
                .iter()
                .zip(variants_tys)
                .map(|(name, ty)| target::EnumVariant {
                    name: name.into(),
                    value: target::EnumVariantKind::Tuple(vec![target::TypeRef::new(ty)]),
                    doc: target::Documentation::default(),
                    since: target::Version::Unknown,
                })
                .collect();

            items.push(target::Item::Enum(target::Enum {
                name: name.into(),
                variants,
                doc: target::Documentation::default(),
                deprecated: None,
                since: target::Version::Unknown,
            }));
        }

        for structure in structures {
            match t.config.structs.remove(&structure.name) {
                Some(CodegenOption::Generate(true | false)) => {
                    // FIXME(wiro): use false to not actually generate the item, makes it mixin only
                    let struct_ = structure
                        .translate(t)
                        .wrap_err_with(|| format!("translating structure {}", structure.name))?;
                    items.push(target::Item::Struct(struct_.clone()));
                    mixins.insert(structure.name.clone(), struct_);
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
                    imports.push(structure.name.clone());
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
                    imports.push(enumeration.name.clone());
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
        for type_alias in type_aliases {
            match t.config.type_aliases.remove(&type_alias.name) {
                Some(CodegenOption::Generate(true | false)) => {
                    let type_alias = type_alias
                        .translate(t)
                        .wrap_err_with(|| format!("translating type alias {}", type_alias.name))?;
                    items.push(target::Item::TypeAlias(type_alias));
                }
                Some(CodegenOption::Checksum(cksum)) => {
                    let hash = {
                        let mut hasher = fasthash::xx::Hasher32::default();
                        type_alias.hash(&mut hasher);
                        format!("{:08x}", hasher.finish())
                    };
                    if hash != *cksum {
                        eprintln!(
                            "Hash mismatch for type alias {}, expected {:?}, got {:?}",
                            type_alias.name, cksum, hash
                        );
                    }
                    imports.push(type_alias.name.clone());
                }
                None => {
                    t.type_aliases_missing.insert(type_alias.name.clone());
                    continue;
                }
            };
        }
        if !t.type_aliases_missing.is_empty() {
            eprintln!("These type aliases are missing. Add them.\n```toml\n[type-aliases]");
            for missing in &t.type_aliases_missing {
                eprintln!("{missing} = true");
            }
            eprintln!("```");
        }

        if !t.anon_missing.is_empty() {
            eprintln!("These anon mappings are missing. Add them.\n```toml\n[anon-mappings]");
            for missing in &t.anon_missing {
                eprintln!("\"{missing}\" = \"crate::Todo\"");
            }
            eprintln!("```");
        }

        // resolve mixins
        for item in &mut items {
            let target::Item::Struct(struct_) = item else {
                continue;
            };

            while let Some(extend) = &struct_.extends.pop() {
                let mixin = mixins.get(extend).unwrap();

                // TODO: could be avoided with simple topological sort
                // TODO: include in the documentation where this field is from
                struct_.extends.extend(mixin.extends.iter().cloned());

                // struct_.fields.extend(mixin.fields.iter().cloned().map(|field| {field.doc = ;field}));
            }
        }

        // for request in requests {
        //     items.push(target::Item::TraitImpl(target::TraitImpl {
        //         interface: "crate::Request".into(),
        //         implementor: request.type_name.clone(),
        //         assoc_types: vec![
        //             (
        //                 "Params".into(),
        //                 request.params.as_ref().unwrap().translate(t)?.unwrap(),
        //             ),
        //             ("Result".into(), request.result.translate(t)?.unwrap()),
        //         ],
        //         assoc_const: vec![(
        //             "METHOD".into(),
        //             target::TypeRef::new("&'static str"),
        //             request.method.clone(),
        //         )],
        //     }));
        // }
        for notification in notifications {
            let params = if let Some(x) = &notification.params {
                x.translate(t)?.unwrap()
            } else {
                TypeRef::new("()")
            };

            let trait_impl = target::TraitImpl {
                interface: "crate::Notification".into(),
                implementor: notification.type_name.clone(),
                assoc_types: vec![("Params".into(), params)],
                assoc_const: vec![(
                    "METHOD".into(),
                    target::TypeRef::new("&'static str"),
                    notification.method.clone(),
                )],
            };
            items.push(target::Item::TraitImpl(trait_impl));
        }

        Ok(target::MetaModel { imports, items })
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
            .map(|ty| ty.clone().into_reference().unwrap().to_string())
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
            let Some(mut ty) = type_
                .translate(t)
                .wrap_err_with(|| format!("while translating property: {name}"))?
            else {
                continue;
            };
            if optional.unwrap_or_default() {
                ty = target::TypeRef::new_generics("Option", &[ty])
            }

            fields.push(target::StructFields {
                name: name.clone(),
                ty,
                doc: documentation.into(),
                since: since.try_into()?,
                deprecated: deprecated.clone(),
            });
        }

        Ok(target::Struct {
            name: name.clone(),
            extends,
            fields,
            doc: documentation.into(),
            since: since.try_into()?,
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
                // TODO
                // bail!("translate int enums")
            }
            schema::EnumerationTypeKind::Uinteger => {
                // TODO
                // bail!("translate uint enums")
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
            variants.push(target::EnumVariant {
                name: name.clone(),
                value: target::EnumVariantKind::Unit,
                doc: documentation.into(),
                since: since.try_into()?,
            });
        }

        Ok(target::Enum {
            name: name.clone(),
            variants,
            doc: documentation.into(),
            since: since.try_into()?,
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
                            value: schema::StructureLiteral { properties, .. }
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
                            t.anon_missing.insert(key);
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

impl TranslateSchema for schema::TypeAlias {
    type Output = target::TypeAlias;
    fn translate(&self, t: &mut Translator) -> eyre::Result<Self::Output> {
        let Self {
            name,
            type_,
            documentation,
            since,
            proposed,
            deprecated,
        } = &self;

        Ok(target::TypeAlias {
            name: name.clone(),
            ty: type_.translate(t)?.unwrap(),
            doc: documentation.into(),
            deprecated: deprecated.clone(),
            since: since.try_into()?,
        })
    }
}
