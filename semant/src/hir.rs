pub(crate) mod function;

pub(crate) use function::{Function, FunctionAstMap};

use errors::FileId;
use std::{path::Path, sync::Arc};
use syntax::{ast, text_of_first_token, AstNode, SmolStr, SyntaxKind, TextRange, T};
pub type Span = TextRange;

#[derive(Debug, Default, Eq, PartialEq, Clone, Hash)]
pub struct SourceFile {
    pub(crate) imports: Vec<Arc<Import>>,
    pub(crate) modules: Vec<Arc<Module>>,
    pub(crate) functions: Vec<Arc<Function>>,
    pub(crate) type_alias: Vec<Arc<TypeAlias>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PatId(pub(crate) u64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypeParamId(pub(crate) u64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParamId(pub(crate) u64);
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]

pub struct StmtId(pub(crate) u64);
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]

pub struct BodyId(pub(crate) u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(SmolStr);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Import {
    pub(crate) id: ImportId,
    pub(crate) segments: Vec<Segment>,
    pub(crate) file: FileId,
    pub(crate) span: Span,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Segment {
    pub(crate) name: NameId,
    pub(crate) nested_imports: Vec<NameId>,
}
/// A symbol is composed of a name and the file it belongs to
/// Symbols with the same name but from different files are not the sames
/// i.e
/// export foo {
///
/// };
/// export foo {
///
/// };
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol(pub(crate) FunctionId, pub(crate) FileId);

impl Name {
    pub fn missing() -> Self {
        Name(SmolStr::new("missing name"))
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ast::IdentType> for Name {
    fn from(name: ast::IdentType) -> Name {
        Name(text_of_first_token(name.syntax()).clone())
    }
}

impl From<ast::Name> for Name {
    fn from(name: ast::Name) -> Name {
        Name(text_of_first_token(name.syntax()).clone())
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<Path> for Name {
    fn as_ref(&self) -> &Path {
        &Path::new(self.0.as_str())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Param {
    pub(crate) pat: PatId,
    pub(crate) ty: TypeId,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TypeParam {
    pub(crate) name: NameId,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct TypeAlias {
    pub(crate) name: NameId,
    pub(crate) type_params: Vec<TypeParamId>,
    pub(crate) ty: TypeId,
    pub(crate) span: Span,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Module {
    pub(crate) id: ModuleId,
    pub(crate) name: NameId,
    pub(crate) file: FileId,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprId(pub(crate) u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId(pub(crate) u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    Bind { name: NameId },
    Placeholder,
    Tuple(Vec<PatId>),
    Literal(LiteralId),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MatchArm {
    pub(crate) pats: Vec<PatId>,
    pub(crate) expr: ExprId,
}
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Literal {
    String(SmolStr),
    Nil,
    True,
    False,
    Int(SmolStr),
    Float(SmolStr),
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Type {
    ParenType(Vec<TypeId>),
    /// An array type with no supplied size is assumed to be dynamic in growth
    /// If the size is present the array has a static size
    ArrayType {
        ty: TypeId,
        size: Option<usize>,
    },
    FnType {
        params: Vec<TypeId>,
        ret: Option<TypeId>,
    },
    Ident(NameId),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Stmt {
    Let {
        pat: PatId,
        initializer: Option<ExprId>,
    },
    Expr(ExprId),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Block(pub Vec<StmtId>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Array(Vec<ExprId>),
    Binary {
        lhs: ExprId,
        op: BinOp,
        rhs: ExprId,
    },
    Block(BlockId),
    Break,
    Call {
        callee: ExprId,
        args: Vec<ExprId>,
    },
    Cast {
        expr: ExprId,
        ty: TypeId,
    },
    Continue,
    If {
        cond: ExprId,
        then_branch: ExprId,
        else_branch: Option<ExprId>,
    },
    Ident(NameId),
    Index {
        base: ExprId,
        index: ExprId,
    },
    While {
        cond: ExprId,
        body: BlockId,
    },
    Literal(LiteralId),
    Paren(ExprId),
    Tuple(Vec<ExprId>),
    Unary {
        op: UnaryOp,
        expr: ExprId,
    },
    Return(Option<ExprId>),
    Match {
        expr: ExprId,
        arms: Vec<MatchArm>,
    },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum BinOp {
    Plus,
    Minus,
    Mult,
    Div,
    And,
    Or,
    LessThan,
    GreaterThan,
    Excl,
    Equal,
    EqualEqual,
    NotEqual,
    LessThanEqual,
    GreaterThanEqual,
    PlusEqual,
    MinusEqual,
    MultEqual,
    DivEqual,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum UnaryOp {
    Minus,
    Excl,
}

macro_rules! create_intern_key {
    ($name:ident) => {
        #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
        pub struct $name(salsa::InternId);
        impl salsa::InternKey for $name {
            fn from_intern_id(v: salsa::InternId) -> Self {
                $name(v)
            }
            fn as_intern_id(&self) -> salsa::InternId {
                self.0
            }
        }
    };
}
create_intern_key!(ClassId);
create_intern_key!(EnumId);
create_intern_key!(TypeAliasId);
create_intern_key!(NameId);
create_intern_key!(FunctionId);
create_intern_key!(TypeId);
create_intern_key!(LiteralId);
create_intern_key!(ModuleId);
create_intern_key!(ImportId);

impl UnaryOp {
    pub(crate) fn from_kind(kind: SyntaxKind) -> Option<UnaryOp> {
        let op = match kind {
            T![-] => UnaryOp::Minus,
            T![!] => UnaryOp::Excl,
            _ => return None,
        };

        Some(op)
    }
}

impl BinOp {
    pub(crate) fn from_kind(kind: SyntaxKind) -> Option<BinOp> {
        let op = match kind {
            T![-] => BinOp::Minus,
            T![+] => BinOp::Plus,
            T![*] => BinOp::Mult,
            T![/] => BinOp::Div,
            T![=] => BinOp::Equal,
            T![&&] => BinOp::And,
            T![||] => BinOp::Or,
            T![<] => BinOp::LessThan,
            T![>] => BinOp::GreaterThan,
            T![==] => BinOp::EqualEqual,
            T![!] => BinOp::Excl,
            T![!=] => BinOp::NotEqual,
            T![<=] => BinOp::LessThanEqual,
            T![>=] => BinOp::GreaterThanEqual,
            T![+=] => BinOp::PlusEqual,
            T![-=] => BinOp::MinusEqual,
            T![*=] => BinOp::MultEqual,
            T![/=] => BinOp::DivEqual,

            _ => return None,
        };
        Some(op)
    }
}

impl Literal {
    pub(crate) fn from_token(token: syntax::SyntaxToken) -> Literal {
        use syntax::SyntaxKind::*;
        let text = token.text().clone();
        let kind = token.kind();
        match kind {
            INT_NUMBER => Literal::Int(text),
            STRING => Literal::String(text),
            FLOAT_NUMBER => Literal::Float(text),
            T![true] => Literal::True,
            T![false] => Literal::False,
            T![nil] => Literal::Nil,
            _ => unreachable!(),
        }
    }
}
