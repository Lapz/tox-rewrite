use crate::hir;
use crate::HirDatabase;
use errors::FileId;
use std::sync::Arc;

use syntax::NameOwner;

pub(crate) fn lower_module_query(
    db: &impl HirDatabase,
    file: FileId,
    mod_id: hir::ModuleId,
) -> Arc<hir::Module> {
    let module = db.lookup_intern_module(mod_id);

    let name = db.intern_name(module.name().map(|name| name.into()).unwrap());

    Arc::new(hir::Module { file, name })
}
