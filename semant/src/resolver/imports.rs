use crate::{hir::ImportId, HirDatabase};
use errors::{FileId, Reporter, WithError};

pub fn resolve_imports_query(
    db: &impl HirDatabase,
    file: FileId,
    import_id: ImportId,
) -> WithError<()> {
    let mut reporter = Reporter::new(file);
    let import = db.lower_import(file, import_id);
    let module_graphs = db.module_graph(file)?;
    let mut nodes = module_graphs.get_node(&file);
    let mut import_err = String::new();

    for segment in &import.segments {
        if let Some(module) = nodes.get(&segment.name) {
            let next_node = module_graphs.try_get_node(&module);

            let mut next_node = next_node.unwrap();

            std::mem::swap(&mut next_node, &mut nodes);

            import_err.push_str(&format!(
                "{}::",
                db.lookup_intern_name(segment.name).as_str()
            ));

            if segment.nested_imports.len() > 0 {
                let exports = db.resolve_exports(*module)?;
                for name in &segment.nested_imports {
                    if !exports.has_export(name) {
                        reporter.error(
                            "Unresolved import",
                            format!(
                                "Couldn't find the import `{}`",
                                format!("{}{}", import_err, db.lookup_intern_name(*name))
                            ),
                            (import.span.start().to_usize(), import.span.end().to_usize()),
                        );
                    }
                }
            }
        } else {
            import_err.push_str(db.lookup_intern_name(segment.name).as_str());

            reporter.error(
                "Unresolved import",
                format!("Couldn't find the import `{}`", import_err),
                (import.span.start().to_usize(), import.span.end().to_usize()),
            )
        }
    }

    if reporter.has_errors() {
        Err(reporter.finish())
    } else {
        Ok(())
    }
}

#[cfg(test)]
#[macro_use]
mod test {

    use crate::create_test;

    create_test!(import_single);
    create_test!(import_many);

    #[should_panic]
    create_test!(import_no_exported);

    create_test!(import_deep);
}
