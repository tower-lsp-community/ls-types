use std::io::{self, Write};

use crate::schema;

fn codegen_struct<'w, 'd>(
    w: &'w mut dyn Write,
    name: &'d str,
    fields: impl Iterator<Item = (&'d str, &'d schema::Type)>,
) -> io::Result<()> {
    writeln!(w, "struct {name} {{")?;
    for (name, ty) in fields {
        write!(w, "\t{name}: ")?;
        write_type(w, ty)?;
        writeln!(w, ", ")?;
    }
    writeln!(w, "}}\n")
}

fn write_type(w: &mut dyn Write, ty: &schema::Type) -> io::Result<()> {
    todo!()
}
