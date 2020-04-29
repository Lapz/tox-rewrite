use crate::{infer::infer_alias, Ctx, HirDatabase};
use errors::{FileId, WithError};

pub fn infer_query(db: &impl HirDatabase, file: FileId) -> WithError<()> {
    let program = db.lower(file)?;

    let mut ctx = Ctx::new(db);

    for alias in &program.type_alias {
        ctx.begin_scope();
        infer_alias(db, file, alias, &mut ctx)?;
        ctx.end_scope();
    }

    Ok(())
}
