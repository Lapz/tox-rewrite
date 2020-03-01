use crate::hir;

use salsa;
use std::sync::Arc;
use syntax::ast;

#[salsa::query_group(InternDatabaseStorage)]
pub trait InternDatabase {
    #[salsa::interned]
    fn intern_function(&self, fn_def: ast::FnDef) -> hir::FunctionId;

    #[salsa::interned]
    fn intern_name(&self, name: hir::Name) -> hir::NameId;

    #[salsa::interned]
    fn intern_class(&self, class_def: ast::ClassDef) -> hir::ClassId;

    #[salsa::interned]
    fn intern_enum(&self, enum_def: ast::EnumDef) -> hir::EnumId;

    #[salsa::interned]
    fn intern_type_alias(&self, type_alias_def: ast::TypeAliasDef) -> hir::TypeAliasId;

    #[salsa::interned]
    fn intern_type(&self, ty: hir::Type) -> hir::TypeId;

    #[salsa::interned]
    fn intern_literal(&self, lit: hir::Literal) -> hir::LiteralId;
}

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: std::fmt::Debug + InternDatabase {
    #[salsa::invoke(crate::lower::lower_function_query)]
    fn lower_function(&self, function: ast::FnDef) -> Arc<hir::Function>;
    #[salsa::invoke(crate::lower::lower_type_alias_query)]
    fn lower_type_alias(&self, alias: ast::TypeAliasDef) -> Arc<hir::TypeAlias>;
    #[salsa::invoke(crate::lower::lower_ast_query)]
    fn lower_ast(&self, source: ast::SourceFile) -> Arc<hir::Program>;
    #[salsa::invoke(crate::resolver::resolve_program_query)]
    fn resolve_program(&self, program: Arc<hir::Program>, reporter: errors::Reporter) -> ();
}
