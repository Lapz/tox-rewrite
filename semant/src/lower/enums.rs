use crate::{
    hir::{self, FunctionAstMap},
    util, HirDatabase,
};

use std::sync::Arc;
use syntax::{ast, NameOwner, TypeParamsOwner, VisibilityOwner};

#[derive(Debug)]
pub(crate) struct EnumDataCollector<DB> {
    db: DB,
    type_param_count: u64,
    type_params: Vec<util::Span<hir::TypeParamId>>,
    ast_map: FunctionAstMap,
}

impl<'a, DB> EnumDataCollector<&'a DB>
where
    DB: HirDatabase,
{
    pub(crate) fn lower_type_param(&mut self, type_param: ast::TypeParam) {
        let name = self.db.intern_name(type_param.name().unwrap().into());

        self.add_type_param(
            &type_param,
            hir::TypeParam {
                name: util::Span::from_ast(name, &type_param),
            },
        );
    }

    pub fn add_type_param(&mut self, ast_node: &ast::TypeParam, type_param: hir::TypeParam) {
        let current = self.type_param_count;

        self.type_param_count += 1;

        let id = hir::TypeParamId(current);

        self.ast_map.insert_type_param(id, type_param);
        self.type_params.push(util::Span::from_ast(id, ast_node));
    }
}

pub(crate) fn lower_enum_query(db: &impl HirDatabase, enum_id: hir::EnumId) -> Arc<()> {
    let enum_ = db.lookup_intern_enum(enum_id);

    let name = util::Span::from_ast(
        db.intern_name(enum_.name().unwrap().into()),
        &enum_.name().unwrap(),
    );

    let mut collector = EnumDataCollector {
        db,
        type_param_count: 0,
        type_params: Vec::new(),
        ast_map: FunctionAstMap::default(),
    };

    let exported = enum_.visibility().is_some();

    if let Some(type_params_list) = enum_.type_param_list() {
        for type_param in type_params_list.type_params() {
            collector.lower_type_param(type_param);
        }
    }

    if let Some(variant_list) = enum_.variant_list() {
        for variant in variant_list.variants() {
            collector.lower_variant(variant);
        }
    }

    let span = enum_.syntax().text_range();

    Arc::new(())
}
