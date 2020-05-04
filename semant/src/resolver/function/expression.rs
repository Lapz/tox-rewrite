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
    ) -> Result<(), ()> {
        let expr = ast_map.expr(expr);

        match expr {
            Expr::Array(_) => {}
            Expr::Binary { lhs, op, rhs } => {}
            Expr::Block(_) => {}
            Expr::Break => {}
            Expr::Call { callee, args } => {}
            Expr::Cast { expr, ty } => {}
            Expr::Continue => {}
            Expr::If {
                cond,
                then_branch,
                else_branch,
            } => {}
            Expr::Ident(_) => {}
            Expr::Index { base, index } => {}
            Expr::While { cond, body } => {}
            Expr::Literal(_) => {}
            Expr::Paren(_) => {}
            Expr::Tuple(_) => {}
            Expr::Unary { op, expr } => {}
            Expr::Return(_) => {}
            Expr::Match { expr, arms } => {}
        }
        Ok(())
    }
}
