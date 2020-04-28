use crate::{infer::infer_alias, Ctx, HirDatabase};
use errors::{FileId, WithError};

pub fn infer_query(db: &impl HirDatabase, file: FileId) -> WithError<()> {
    let program = db.lower(file)?;

    let mut ctx = Ctx::new(db);

    println!("before:\n{:#?}", ctx);

    for alias in &program.type_alias {
        ctx.begin_scope();
        infer_alias(db, file, alias, &mut ctx)?;
        ctx.end_scope();
    }

    println!("after:\n{:#?}", ctx);

    Ok(())
}
