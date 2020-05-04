use super::source_file::{FunctionData, State};
use crate::{
    hir::{self, FunctionId, NameId, TypeId},
    infer::{Type, TypeCon},
    util, Ctx, HirDatabase,
};
use errors::{FileId, Reporter, WithError};
use hir::PatId;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub(crate) struct ResolverDataCollector<DB> {
    db: DB,
    pub(crate) ctx: Ctx,
    pub(crate) reporter: Reporter,
    pub(crate) items: HashSet<hir::NameId>,
    pub(crate) exported_items: HashSet<util::Span<hir::NameId>>,
    pub(crate) function_data: HashMap<hir::NameId, FunctionData>,
}

impl<'a, DB> ResolverDataCollector<&'a DB>
where
    DB: HirDatabase,
{
    pub fn finish(self) -> (Ctx, Reporter) {
        (self.ctx, self.reporter)
    }
    pub(crate) fn begin_scope(&mut self) {
        self.ctx.begin_scope();
    }

    pub(crate) fn end_scope(&mut self) {
        self.ctx.end_scope();
    }

    pub(crate) fn add_function(&mut self, name_id: util::Span<NameId>, exported: bool) {
        if self.items.contains(&name_id.item) {
            let name = self.db.lookup_intern_name(name_id.item);

            self.reporter.error(
                format!("The name `{}` is defined multiple times", name),
                "",
                (name_id.start().to_usize(), name_id.end().to_usize()),
            )
        } else {
            if exported {
                self.exported_items.insert(name_id);
            }

            self.function_data.insert(name_id.item, FunctionData::new());
        }
    }

    pub(crate) fn resolve_function_scope(&self, name: NameId) -> usize {
        self.function_data[&name].peek()
    }

    pub(crate) fn add_local(&mut self, fn_name: NameId, param: util::Span<NameId>) {
        let scope = self.resolve_function_scope(fn_name);

        if self.function_data[&fn_name].scopes[scope].contains_key(&param.item) {
            let msg = format!(
                "The identifier `{}` has already been declared.",
                self.db.lookup_intern_name(param.item)
            );

            self.reporter
                .warn(msg, "", (param.start().to_usize(), param.end().to_usize()))
        }

        let function_data = self.function_data.get_mut(&fn_name).unwrap();
        function_data.scopes[scope].insert(
            param.item,
            util::Span::new(State::Declared, param.start(), param.end()),
        );
    }

    /// Resolve a  pattern
    /// A pattern can occur in a fn param def
    /// or in a let statement

    pub(crate) fn resolve_pattern(
        &mut self,
        fn_name: NameId,
        pat_id: &util::Span<PatId>,
        ast_map: &hir::FunctionAstMap,
    ) -> Result<(), ()> {
        let pat = ast_map.pat(&pat_id.item);

        match pat {
            hir::Pattern::Bind { name } => self.add_local(fn_name, *name),
            hir::Pattern::Tuple(patterns) => {
                for pat in patterns {
                    if let Err(_) = self.resolve_pattern(fn_name, pat, ast_map) {
                        continue;
                    }
                }
            }
            hir::Pattern::Placeholder | hir::Pattern::Literal(_) => {}
        }

        Ok(())
    }

    pub(crate) fn resolve_type(&mut self, id: &util::Span<TypeId>) -> Result<Type, ()> {
        let ty = self.db.lookup_intern_type(id.item);

        match ty {
            hir::Type::ParenType(types) => {
                let mut signature = vec![];
                for id in &types {
                    signature.push(self.resolve_type(id)?)
                }

                Ok(Type::Tuple(signature))
            }

            hir::Type::ArrayType { ty, size } => Ok(Type::Con(TypeCon::Array {
                ty: Box::new(self.resolve_type(&ty)?),
                size,
            })),
            hir::Type::FnType { params, ret } => {
                let mut signature = vec![];

                for id in &params {
                    signature.push(self.resolve_type(id)?)
                }

                if let Some(returns) = &ret {
                    signature.push(self.resolve_type(returns)?)
                } else {
                    signature.push(Type::Con(TypeCon::Void))
                }

                Ok(Type::App(signature))
            }
            hir::Type::Poly { name, type_args } => {
                let ty = match self.ctx.get_type(&name) {
                    Some(ty) => ty,
                    None => {
                        let span = (id.start().to_usize(), id.end().to_usize());
                        self.reporter.error(
                            format!(
                                "Use of undefined type `{}`",
                                self.db.lookup_intern_name(name)
                            ),
                            "",
                            span,
                        );

                        return Err(());
                    }
                };

                for arg in &type_args {
                    if let Err(()) = self.resolve_type(arg) {
                        continue;
                    }
                }

                Ok(ty)
            }
            hir::Type::Ident(name) => {
                if let Some(ty) = self.ctx.get_type(&name) {
                    return Ok(ty);
                }

                let span = (id.start().to_usize(), id.end().to_usize());
                self.reporter.error(
                    format!(
                        "Use of undefined type `{}`",
                        self.db.lookup_intern_name(name)
                    ),
                    "",
                    span,
                );

                Err(())
            }
        }
    }
}

pub fn resolve_file_query(db: &impl HirDatabase, file: FileId) -> WithError<()> {
    let source_file = db.lower(file)?;

    let reporter = Reporter::new(file);

    let ctx = Ctx::new(db);

    let mut collector = ResolverDataCollector {
        db,
        reporter,
        ctx,
        items: HashSet::new(),
        exported_items: HashSet::new(),
        function_data: HashMap::new(),
    };

    for alias in &source_file.type_alias {
        if let Err(_) = collector.resolve_alias(alias) {
            continue;
        };
    }

    for function in &source_file.functions {
        if let Err(_) = collector.resolve_function(function) {
            continue;
        }
    }

    let (_ctx, reporter) = collector.finish();

    if reporter.has_errors() {
        Err(reporter.finish())
    } else {
        Ok(())
    }
}
