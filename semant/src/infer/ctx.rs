use super::stacked_map::StackedMap;
use crate::{
    hir::{Name, NameId},
    infer::ty::{EnumVariant, Type, TypeCon, TypeVar},
    HirDatabase,
};
use errors::{FileId, WithError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ctx {
    types: StackedMap<NameId, Type>,
    tvar_count: u32,
}

impl Ctx {
    pub fn new(db: &impl HirDatabase) -> Self {
        let mut types = StackedMap::new();

        types.insert(db.intern_name(Name::new("i32")), Type::Con(TypeCon::Int));
        types.insert(db.intern_name(Name::new("f32")), Type::Con(TypeCon::Float));
        types.insert(db.intern_name(Name::new("bool")), Type::Con(TypeCon::Bool));
        types.insert(db.intern_name(Name::new("void")), Type::Con(TypeCon::Void));
        types.insert(db.intern_name(Name::new("string")), Type::Con(TypeCon::Str));

        let result_name = db.intern_name(Name::new("Result"));
        types.insert(
            result_name,
            Type::Enum(
                result_name,
                vec![
                    EnumVariant {
                        tag: 0,
                        inner: Some(Type::Var(TypeVar::from(0))), // Ok(T)
                    },
                    EnumVariant {
                        tag: 1,
                        inner: Some(Type::Var(TypeVar::from(1))), // Err(U)
                    },
                ],
            ),
        );

        Self {
            types,
            tvar_count: 2,
        }
    }

    pub(crate) fn type_var(&mut self) -> TypeVar {
        let tv = TypeVar::from(self.tvar_count);
        self.tvar_count += 1;

        tv
    }
}

pub fn context_query(db: &impl HirDatabase, file: FileId) -> WithError<Ctx> {
    let program = db.lower(file)?;

    let mut ctx = Ctx::new(db);

    for alias in &program.type_alias {}
    unimplemented!()
}
