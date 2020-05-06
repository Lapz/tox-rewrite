use crate::{
    hir::{Expr, ExprId, FunctionAstMap, NameId},
    resolver::data::ResolverDataCollector,
    util, HirDatabase,
};

impl<'a, DB> ResolverDataCollector<&'a DB>
where
    DB: HirDatabase,
{
    pub(crate) fn resolve_expression(
        &mut self,
        fn_name: &util::Span<NameId>,
        expr: &ExprId,
        ast_map: &FunctionAstMap,
    ) {
        let expr = ast_map.expr(expr);

        match expr {
            Expr::Array(exprs) | Expr::Tuple(exprs) => {
                exprs
                    .iter()
                    .for_each(|id| self.resolve_expression(fn_name, id, ast_map));
            }
            Expr::Binary { lhs, rhs, .. } => {
                self.resolve_expression(fn_name, lhs, ast_map);
                self.resolve_expression(fn_name, rhs, ast_map);
            }
            Expr::Block(block_id) => {
                let block = ast_map.block(block_id);

                self.begin_function_scope(fn_name.item);

                block
                    .0
                    .iter()
                    .for_each(|id| self.resolve_statement(fn_name, id, ast_map));

                self.end_function_scope(fn_name.item);
            }
            Expr::Break | Expr::Continue => {}
            Expr::Call {
                callee,
                args,
                type_args,
            } => {
                self.resolve_expression(fn_name, callee, ast_map);

                match ast_map.expr(callee) {
                    Expr::Ident(ident) => {
                        if let Some(ty) = self.ctx.get_type(&ident.item) {
                            match ty {
                                crate::infer::Type::Poly(ref ty_vars, _) => {
                                    if type_args.item.len() != ty_vars.len() {
                                        let msg = format!(
                                            "Expected `{}` type arguments found `{}`",
                                            ty_vars.len(),
                                            type_args.item.len()
                                        );

                                        let info = if type_args.item.len() > ty_vars.len() {
                                            "Too many type arguments supplied, try removing some"
                                        } else {
                                            "Too few type arguments supplied, try adding some more"
                                        };

                                        self.reporter.error(
                                            msg,
                                            info,
                                            (
                                                type_args.start().to_usize(),
                                                type_args.start().to_usize(),
                                            ),
                                        )
                                    }
                                }

                                _ => (),
                            }
                        }
                    }
                    e => println!("{:?}", e),
                };

                args.iter()
                    .for_each(|id| self.resolve_expression(fn_name, id, ast_map));

                type_args.item.iter().for_each(|ty| {
                    let _ = self.resolve_type(ty);
                })
            }
            Expr::Cast { expr, ty } => {
                self.resolve_expression(fn_name, expr, ast_map);
                let _ = self.resolve_type(ty);
            }

            Expr::If {
                cond,
                then_branch,
                else_branch,
            } => {
                self.resolve_expression(fn_name, cond, ast_map);
                self.resolve_expression(fn_name, then_branch, ast_map);

                if let Some(else_branch) = else_branch {
                    self.resolve_expression(fn_name, else_branch, ast_map)
                }
            }
            Expr::Ident(name) => {
                if self.local_is_declared(&fn_name.item, name) {
                    let msg = format!(
                        "Cannot read local name `{}` in its own initializer.",
                        self.db.lookup_intern_name(name.item)
                    );

                    self.reporter
                        .error(msg, "", (name.start().to_usize(), name.end().to_usize()))
                }

                self.resolve_local(&fn_name.item, name)
            }
            Expr::Index { base, index } => {
                self.resolve_expression(fn_name, base, ast_map);
                self.resolve_expression(fn_name, index, ast_map)
            }
            Expr::While { cond, body } => {
                self.resolve_expression(fn_name, cond, ast_map);

                let block = ast_map.block(body);

                self.begin_function_scope(fn_name.item);

                block
                    .0
                    .iter()
                    .for_each(|id| self.resolve_statement(fn_name, id, ast_map));

                self.end_function_scope(fn_name.item);
            }
            Expr::Literal(_) => {}
            Expr::Paren(expr) => self.resolve_expression(fn_name, expr, ast_map),

            Expr::Unary { expr, .. } => self.resolve_expression(fn_name, expr, ast_map),
            Expr::Return(expr) => {
                if let Some(expr) = expr {
                    self.resolve_expression(fn_name, expr, ast_map)
                }
            }
            Expr::Match { expr, arms } => {
                self.resolve_expression(fn_name, expr, ast_map);

                arms.iter().for_each(|arm| {
                    arm.pats
                        .iter()
                        .for_each(|pat_id| self.resolve_pattern(fn_name.item, pat_id, ast_map));
                    self.resolve_expression(fn_name, &arm.expr, ast_map)
                })
            }
        }
    }
}
