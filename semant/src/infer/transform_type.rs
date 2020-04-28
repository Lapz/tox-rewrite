use crate::{
    db::HirDatabase,
    hir,
    infer::{self, Ctx},
};
use errors::{FileId, Reporter, WithError};

pub(crate) fn transform_type(
    db: &impl HirDatabase,
    file: FileId,
    ty: hir::TypeId,
    ctx: &mut Ctx,
) -> WithError<infer::Type> {
    let reporter = Reporter::new(file);

    let ty = db.lookup_intern_type(ty);

    match ty {
        hir::Type::ParenType(types) => {
            let mut signature = vec![];

            for id in &types {
                signature.push(transform_type(db, file, *id, ctx)?)
            }

            Ok(infer::Type::Tuple(signature))
        }

        hir::Type::ArrayType { ty, size } => Ok(infer::Type::Con(infer::TypeCon::Array {
            ty: Box::new(transform_type(db, file, ty, ctx)?),
            size,
        })),
        hir::Type::FnType { params, ret } => {
            let mut signature = vec![];

            for id in &params {
                signature.push(transform_type(db, file, *id, ctx)?)
            }

            if let Some(returns) = ret {
                signature.push(transform_type(db, file, returns, ctx)?)
            } else {
                signature.push(infer::Type::Con(infer::TypeCon::Void))
            }

            Ok(infer::Type::App(signature))
        }
        hir::Type::Ident(name) => {
            //  Ok(infer::Type::Con(infer::TypeCon::Int))

            // if let Some(ty) =

            // let msg =
            unimplemented!()
        }
    }
}
