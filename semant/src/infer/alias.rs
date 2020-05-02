use crate::hir;
use crate::infer::{transform_type, Ctx, Type};
use crate::HirDatabase;
use errors::{FileId, WithError};

// When inferring a type alias
// We add the type param as their own types to the ctx
// This is so when transform the ast type on the rhs
// we don't get any undefined type errors

pub fn infer_alias(
    db: &impl HirDatabase,
    file: FileId,
    alias: &hir::TypeAlias,
    ctx: &mut Ctx,
) -> WithError<()> {
    let name = alias.name;

    let poly_tvs = alias
        .type_params
        .iter()
        .map(|type_param| {
            let tv = ctx.type_var();

            // let name = db.lookup_intern_name(type_param.item.);

            let name = unimplemented!();

            ctx.insert_type(name, Type::Var(tv));

            tv
        })
        .collect::<Vec<_>>();

    let ty = transform_type(db, file, &alias.ty, ctx)?;

    ctx.insert_type(name.item, Type::Poly(poly_tvs, Box::new(ty)));
    Ok(())
}
