use super::data::ResolverDataCollector;
use crate::{
    hir::Function,
    infer::{Type, TypeCon},
    HirDatabase,
};
impl<'a, DB> ResolverDataCollector<&'a DB>
where
    DB: HirDatabase,
{
    pub(crate) fn resolve_function(&mut self, function: &Function) -> Result<(), ()> {
        let name = function.name;

        self.add_function(name, function.exported);

        self.begin_scope();

        let poly_tvs = function
            .type_params
            .iter()
            .map(|type_param| {
                let type_param = function.ast_map.type_param(&type_param.item);

                let tv = self.ctx.type_var();

                self.ctx.insert_type(type_param.name, Type::Var(tv));

                tv
            })
            .collect::<Vec<_>>();

        let mut signature = Vec::new();

        for param in &function.params {
            let param = function.ast_map.param(&param.item);

            let _ = self.resolve_pattern(function.name.item, &param.pat, &function.ast_map);

            signature.push(self.resolve_type(&param.ty)?);
        }

        if let Some(returns) = &function.returns {
            signature.push(self.resolve_type(&returns)?)
        } else {
            signature.push(Type::Con(TypeCon::Void))
        }

        self.ctx.insert_type(
            name.item,
            Type::Poly(poly_tvs, Box::new(Type::App(signature))),
        );

        self.end_scope();

        Ok(())
    }
}
