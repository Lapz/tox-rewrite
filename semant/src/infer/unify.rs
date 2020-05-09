use super::Type;
use crate::HirDatabase;

pub fn unify_query(db: &impl HirDatabase, lhs: &Type, rhs: &Type) -> Result<(), ()> {
    Ok(())
}
