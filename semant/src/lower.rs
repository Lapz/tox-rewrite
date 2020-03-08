mod alias;
mod function;
mod module;
use crate::db::HirDatabase;
use crate::hir;
pub(crate) use alias::lower_type_alias_query;
use errors::{FileId, WithError};
pub(crate) use function::lower_function_query;
pub(crate) use module::lower_module_query;
use std::sync::Arc;
use syntax::{FnDefOwner, ModuleDefOwner, TypeAliasDefOwner};

pub(crate) fn lower_query(db: &impl HirDatabase, file: FileId) -> WithError<Arc<hir::SourceFile>> {
    let source = db.parse(file)?;
    let mut program = hir::SourceFile::default();

    for module in source.modules() {
        let id = db.intern_module(module);
        program.modules.push(db.lower_module(file, id));
    }

    for type_alias in source.type_alias() {
        let id = db.intern_type_alias(type_alias);

        program.type_alias.push(db.lower_type_alias(id));
    }

    for function in source.functions() {
        let id = db.intern_function(function);
        program.functions.push(db.lower_function(file, id));
    }

    Ok(Arc::new(program))
}
