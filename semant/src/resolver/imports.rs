use crate::{hir::ImportId, HirDatabase};
use errors::{FileId, Reporter, WithError};

pub fn resolve_imports_query(
    db: &impl HirDatabase,
    file: FileId,
    import_id: ImportId,
) -> WithError<()> {
    let mut reporter = Reporter::new(file);
    let import = db.lower_import(file, import_id);
    Ok(())
}
