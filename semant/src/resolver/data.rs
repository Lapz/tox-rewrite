use crate::{
    hir::{self, TypeId},
    infer::{Type, TypeCon},
    util, Ctx, HirDatabase,
};
use errors::{FileId, Reporter, WithError};
use std::collections::HashSet;

#[derive(Debug)]
pub(crate) struct ResolverDataCollector<DB> {
    db: DB,
    pub(crate) ctx: Ctx,
    pub(crate) reporter: Reporter,
    pub(crate) items: HashSet<hir::NameId>,
    pub(crate) exported_items: HashSet<hir::NameId>,
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
