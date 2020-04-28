use crate::hir;
use crate::infer::{transform_type, Ctx, Type};
use crate::HirDatabase;
use errors::{FileId, WithError};

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
        .map(|_| ctx.type_var())
        .collect::<Vec<_>>();

    let ty = transform_type(db, file, alias.ty, ctx)?;

    ctx.insert_type(name, Type::Poly(poly_tvs, Box::new(ty)));
    Ok(())
}
