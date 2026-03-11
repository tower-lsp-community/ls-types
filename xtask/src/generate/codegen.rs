use std::io::{self, Write};

use heck::{ToSnekCase, ToUpperCamelCase};

use crate::generate::target;

struct CodegenCtx<'w> {
    w: &'w mut dyn Write,
    default_derives: String,
}

pub fn codegen_schema<'w, 'd>(
    w: &'w mut dyn Write,
    target::MetaModel {
        default_derives,
        imports,
        items,
    }: &'d target::MetaModel,
) -> io::Result<()> {
    let mut cx = CodegenCtx {
        w,
        default_derives: format!("#[derive({})]", default_derives.join(", ")),
    };

    writeln!(cx.w, "use serde::{{Deserialize, Serialize}};")?;
    writeln!(cx.w, "use crate::manual::{{{}}};", imports.join(", "))?;
    writeln!(cx.w)?;

    for item in items {
        item.codegen(&mut cx)?;
        writeln!(cx.w)?;
    }
    Ok(())
}

trait Codegen<'w> {
    fn codegen(&self, cx: &mut CodegenCtx<'w>) -> io::Result<()>;
}

impl Codegen<'_> for target::Item {
    fn codegen(&self, cx: &mut CodegenCtx<'_>) -> io::Result<()> {
        match self {
            Self::Struct(struct_) => struct_.codegen(cx)?,
            Self::Enum(enum_) => enum_.codegen(cx)?,
            Self::TypeAlias(type_alias) => type_alias.codegen(cx)?,
            Self::TraitImpl(trait_impl) => trait_impl.codegen(cx)?,
        }
        Ok(())
    }
}

impl Codegen<'_> for target::Struct {
    fn codegen(&self, cx: &mut CodegenCtx<'_>) -> io::Result<()> {
        let Self {
            name,
            extends,
            fields,
            doc,
            deprecated,
            since,
        } = &self;

        doc.codegen(cx)?;
        codegen_deprecated(cx, deprecated)?;
        writeln!(cx.w, "{}", cx.default_derives)?;
        writeln!(cx.w, "pub struct {name} {{")?;
        for field in fields {
            field.codegen(cx)?;
        }
        writeln!(cx.w, "}}\n")?;
        Ok(())
    }
}

impl Codegen<'_> for target::StructFields {
    fn codegen(&self, cx: &mut CodegenCtx<'_>) -> io::Result<()> {
        let Self {
            name,
            ty,
            doc,
            since,
            deprecated,
        } = &self;

        let name = match name.as_str() {
            "type" => "r#type",
            name => &name.to_snek_case(),
        };

        doc.codegen(cx)?;
        codegen_deprecated(cx, deprecated)?;
        writeln!(cx.w, "\tpub {name}: {},", &ty)?;

        Ok(())
    }
}

impl Codegen<'_> for target::Enum {
    fn codegen(&self, cx: &mut CodegenCtx<'_>) -> io::Result<()> {
        let Self {
            name,
            variants,
            doc,
            deprecated,
            since,
        } = &self;

        doc.codegen(cx)?;
        codegen_deprecated(cx, deprecated)?;
        writeln!(cx.w, "{}", cx.default_derives)?;
        writeln!(cx.w, "pub enum {name} {{")?;
        for variant in variants {
            variant.codegen(cx)?;
        }
        writeln!(cx.w, "}}\n")?;

        Ok(())
    }
}

impl Codegen<'_> for target::EnumVariant {
    fn codegen(&self, cx: &mut CodegenCtx<'_>) -> io::Result<()> {
        let Self {
            name,
            value,
            doc,
            since,
        } = &self;

        doc.codegen(cx)?;
        write!(cx.w, "{}", name.to_upper_camel_case())?;
        match value {
            target::EnumVariantKind::Unit => {}
            target::EnumVariantKind::Tuple(members) => {
                write!(
                    cx.w,
                    "({})",
                    members
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
            }
        }
        writeln!(cx.w, ",")
    }
}

impl Codegen<'_> for target::TypeAlias {
    fn codegen(&self, cx: &mut CodegenCtx<'_>) -> io::Result<()> {
        let Self {
            name,
            ty,
            doc,
            since,
            deprecated,
        } = &self;

        doc.codegen(cx)?;
        codegen_deprecated(cx, deprecated)?;
        writeln!(cx.w, "pub type {name} = {ty};")
    }
}

impl Codegen<'_> for target::TraitImpl {
    fn codegen(&self, cx: &mut CodegenCtx<'_>) -> io::Result<()> {
        let Self {
            interface,
            implementor,
            assoc_types,
            assoc_const,
        } = &self;

        writeln!(cx.w, "impl {interface} for {implementor} {{")?;
        for (name, ty) in assoc_types {
            writeln!(cx.w, "type {name} = {ty};")?;
        }
        for (name, ty, value) in assoc_const {
            writeln!(cx.w, "const {name}: {ty} = \"{value}\";")?;
        }
        writeln!(cx.w, "}}\n")?;
        Ok(())
    }
}

impl Codegen<'_> for target::Documentation {
    fn codegen(&self, cx: &mut CodegenCtx<'_>) -> io::Result<()> {
        for line in self.0.lines() {
            writeln!(cx.w, "/// {line}")?;
        }
        Ok(())
    }
}

fn codegen_deprecated<'w>(cx: &mut CodegenCtx<'w>, deprecated: &Option<String>) -> io::Result<()> {
    if let Some(reason) = deprecated {
        writeln!(cx.w, "#[deprecated = \"{reason}\"]")?;
    }
    Ok(())
}
