use super::{
    data::{ItemKind, ResolverDataCollector},
    TypeKind,
};
use crate::{hir::Class, infer::Type, HirDatabase};
use std::collections::HashMap;

impl<'a, DB> ResolverDataCollector<&'a DB>
where
    DB: HirDatabase,
{
    pub fn resolve_class(&mut self, class: &Class) -> Result<(), ()> {
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

        let mut methods = HashMap::new();

        // forward declare methods

        for method in &class.methods {
            self.add_item(method.name, ItemKind::Function, method.exported);
            if let Ok(sig) = self.resolve_function_signature(method) {
                methods.insert(method.name.item, sig);
            } else {
                continue;
            }
        }

        for method in &class.methods {
            if let Err(_) = self.resolve_function(method) {
                continue;
            }
        }

        self.end_scope();

        self.insert_type(
            &class.name,
            Type::Poly(poly_tvs, Box::new(Type::Class { fields, methods })),
            TypeKind::Class,
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::create_test;

    create_test!(basic_class);

    create_test!(exported_class);
}
