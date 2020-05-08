use super::{
    data::{ItemKind, ResolverDataCollector},
    TypeKind,
};
use crate::{
    hir::{Class, Function, FunctionAstMap, NameId, StmtId},
    infer::{Type, TypeCon},
    util, HirDatabase,
};
use std::collections::HashMap;

impl<'a, DB> ResolverDataCollector<&'a DB>
where
    DB: HirDatabase,
{
    pub fn resolve_class(&mut self, class: &Class) -> Result<(), ()> {
        let name = class.name;

        self.begin_scope();

        let mut poly_tvs = Vec::new();

        for type_param in &class.type_params {
            let type_param = class.ast_map.type_param(&type_param.item);

            let tv = self.ctx.type_var();

            self.insert_type(&type_param.name, Type::Var(tv), TypeKind::Type)?;

            poly_tvs.push(tv);
        }

        let mut fields = HashMap::new();

        for field in &class.fields {
            if fields.contains_key(&field.item.property.item) {
                let msg = format!(
                    "Duplicate property `{}`",
                    self.db.lookup_intern_name(field.item.property.item)
                );

                let span = field.item.property.as_reporter_span();

                self.reporter.error(msg, "", span);

                continue;
            }

            let ty = match self.resolve_type(&field.item.ty) {
                Ok(ty) => ty,
                Err(_) => continue,
            };

            fields.insert(field.item.property.item, ty);
        }

        let mut methods: Vec<Type> = Vec::new();

        self.begin_scope();

        // forward declare methods

        for method in &class.methods {
            self.add_item(method.name, ItemKind::Function, method.exported)
        }

        for method in &class.methods {
            if let Err(_) = self.resolve_function(method) {
                continue;
            }

            let ty = self.ctx.get_type(&method.name.item).unwrap();

            methods.push(ty);

            // self.ctx.remove_type();
        }

        self.end_scope();

        Ok(())
    }
}
