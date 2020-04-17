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
    println!("{:?} {:#?} file {:?}", nodes, import, file);
    for segment in &import.segments {
        println!("{:?}", db.lookup_intern_name(segment.name));
        if let Some(module) = nodes.get(&segment.name) {
            let next_node = module_graphs.try_get_node(&module);

            let mut next_node = next_node.unwrap();

            std::mem::swap(&mut next_node, &mut nodes);
        } else {
            reporter.error(
                "Unresolved import",
                format!(
                    "Expected a module to be located at {}",
                    db.lookup_intern_file(file).display()
                ),
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
