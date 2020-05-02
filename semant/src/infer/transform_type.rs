use crate::{
    db::HirDatabase,
    hir,
    infer::{self, Ctx},
    util,
};
use errors::{FileId, Reporter, WithError};

pub(crate) fn transform_type(
    db: &impl HirDatabase,
    file: FileId,
    id: &util::Span<hir::TypeId>,
    ctx: &mut Ctx,
) -> WithError<infer::Type> {
    let mut reporter = Reporter::new(file);

    let ty = db.lookup_intern_type(id.item);

    println!("{:?}", ty);

    match ty {
        hir::Type::ParenType(types) => {
            let mut signature = vec![];

            for id in &types {
                signature.push(transform_type(db, file, id, ctx)?)
            }

            Ok(infer::Type::Tuple(signature))
        }

        hir::Type::ArrayType { ty, size } => Ok(infer::Type::Con(infer::TypeCon::Array {
            ty: Box::new(transform_type(db, file, &ty, ctx)?),
            size,
        })),
        hir::Type::FnType { params, ret } => {
            let mut signature = vec![];

            for id in &params {
                signature.push(transform_type(db, file, id, ctx)?)
            }

            if let Some(returns) = ret {
                signature.push(transform_type(db, file, &returns, ctx)?)
            } else {
                signature.push(infer::Type::Con(infer::TypeCon::Void))
            }

            Ok(infer::Type::App(signature))
        }
        hir::Type::Ident(name) => {
            if let Some(ty) = ctx.get_type(&name) {
                return Ok(ty.clone());
            }

            let span = (id.start().to_usize(), id.end().to_usize());
            reporter.error(
                format!("Use of undefined type `{}`", db.lookup_intern_name(name)),
                "",
                span,
            );

            Err(reporter.finish())
        }
        hir::Type::Poly { name, type_args } => {
            for ty in type_args.iter() {
                match transform_type(db, file, ty, ctx) {
                    Ok(_) => {}
                    Err(e) => reporter.extend(e),
                }
            }

            let span = (id.start().to_usize(), id.end().to_usize());
            reporter.error(
                format!("Use of undefined type `{}`", db.lookup_intern_name(name)),
                "",
                span,
            );

            Err(reporter.finish())

            // if let Some(ty) = ctx.get_type(&name) {
            //     return Ok(ty.clone());
            // }
        }
    }
}
