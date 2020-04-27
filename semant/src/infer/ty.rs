use crate::hir::NameId;

/// A type var represent a variable that could be a type
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TypeVar(pub(crate) u32);
/// A unique identifier that is used to distinguish to types with the exact some fields
/// i.e struct Foo {} && struct Bar {} we treat them differently
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Unique(pub(crate) u32);

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TypeCon {
    Bool,
    Float,
    Int,
    Str,
    Void,
    Array(Box<TypeCon>),
}

/// All of of our base types
///
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Type {
    /// (x |-> y)
    /// Last type in a app is the return
    /// type of i32 => App(Con(Int))
    App(Vec<Type>),
    Poly(Vec<TypeVar>, Box<Type>),
    Var(TypeVar),
    Con(TypeCon),
    Enum(NameId, Vec<EnumVariant>),
}

/// Represent an enum variant
/// ```ignore
/// Foo::Bar => Variant {
//      tag:0, // the number it was declared at
///     inner:None // if it dosen't have an inner type i.e Ok(foo)
///  }
/// ```

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EnumVariant {
    pub tag: u32,
    pub inner: Option<Type>,
}

impl From<u32> for TypeVar {
    fn from(i: u32) -> Self {
        Self(i)
    }
}
