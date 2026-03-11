use std::io::{self, Write};

use heck::{ToSnekCase, ToUpperCamelCase};

use crate::generate::target;

pub fn codegen_schema<'w, 'd>(
    w: &'w mut dyn Write,
    schema: &'d target::MetaModel,
) -> io::Result<()> {
    schema.codegen(w)
}

trait Codegen<'w> {
    fn codegen(&self, w: &'w mut dyn Write) -> io::Result<()>;
}

impl Codegen<'_> for target::MetaModel {
    fn codegen(&self, w: &'_ mut dyn Write) -> io::Result<()> {
        let Self { imports, items } = &self;

        writeln!(w, "use crate::manual::{{{}}};\n", imports.join(", "))?;
        for item in items {
            item.codegen(w)?;
            writeln!(w)?;
        }
        Ok(())
    }
}

impl Codegen<'_> for target::Item {
    fn codegen(&self, w: &'_ mut dyn Write) -> io::Result<()> {
        match self {
            Self::Struct(struct_) => struct_.codegen(w)?,
            Self::Enum(enum_) => enum_.codegen(w)?,
            Self::TypeAlias(type_alias) => type_alias.codegen(w)?,
            Self::TraitImpl(trait_impl) => trait_impl.codegen(w)?,
        }
        Ok(())
    }
}

impl Codegen<'_> for target::Struct {
    fn codegen(&self, w: &'_ mut dyn Write) -> io::Result<()> {
        let Self {
            name,
            extends,
            fields,
            doc,
            deprecated,
            since,
        } = &self;

        doc.codegen(w)?;
        codegen_deprecated(w, deprecated)?;
        writeln!(w, "pub struct {name} {{")?;
        for field in fields {
            field.codegen(w)?;
        }
        writeln!(w, "}}\n")?;
        Ok(())
    }
}

impl Codegen<'_> for target::StructFields {
    fn codegen(&self, w: &'_ mut dyn Write) -> io::Result<()> {
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

        doc.codegen(w)?;
        codegen_deprecated(w, deprecated)?;
        writeln!(w, "\tpub {name}: {},", &ty)?;

        Ok(())
    }
}

impl Codegen<'_> for target::Enum {
    fn codegen(&self, w: &'_ mut dyn Write) -> io::Result<()> {
        let Self {
            name,
            variants,
            doc,
            deprecated,
            since,
        } = &self;

        doc.codegen(w)?;
        codegen_deprecated(w, deprecated)?;
        writeln!(w, "pub enum {name} {{")?;
        for variant in variants {
            variant.codegen(w)?;
        }
        writeln!(w, "}}\n")?;

        Ok(())
    }
}

impl Codegen<'_> for target::EnumVariant {
    fn codegen(&self, w: &'_ mut dyn Write) -> io::Result<()> {
        let Self {
            name,
            value,
            doc,
            since,
        } = &self;

        doc.codegen(w)?;
        write!(w, "{}", name.to_upper_camel_case())?;
        match value {
            target::EnumVariantKind::Unit => {}
            target::EnumVariantKind::Tuple(members) => {
                write!(
                    w,
                    "({})",
                    members
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
            }
        }
        writeln!(w, ",")
    }
}

impl Codegen<'_> for target::TypeAlias {
    fn codegen(&self, w: &'_ mut dyn Write) -> io::Result<()> {
        let Self {
            name,
            ty,
            doc,
            since,
            deprecated,
        } = &self;

        doc.codegen(w)?;
        codegen_deprecated(w, deprecated)?;
        writeln!(w, "pub type {name} = {ty};")
    }
}

impl Codegen<'_> for target::TraitImpl {
    fn codegen(&self, w: &'_ mut dyn Write) -> io::Result<()> {
        let Self {
            interface,
            implementor,
            assoc_types,
            assoc_const,
        } = &self;

        writeln!(w, "impl {interface} for {implementor} {{")?;
        for (name, ty) in assoc_types {
            writeln!(w, "type {name} = {ty};")?;
        }
        for (name, ty, value) in assoc_const {
            writeln!(w, "const {name}: {ty} = \"{value}\";")?;
        }
        writeln!(w, "}}\n")?;
        Ok(())
    }
}

impl Codegen<'_> for target::Documentation {
    fn codegen(&self, w: &'_ mut dyn Write) -> io::Result<()> {
        for line in self.0.lines() {
            writeln!(w, "/// {line}")?;
        }
        Ok(())
    }
}

fn codegen_deprecated<'w>(w: &'w mut dyn Write, deprecated: &Option<String>) -> io::Result<()> {
    if let Some(reason) = deprecated {
        writeln!(w, "#[deprecated = \"{reason}\"]")?;
    }
    Ok(())
}
