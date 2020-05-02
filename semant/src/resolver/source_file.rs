use crate::db::HirDatabase;
use crate::{hir, util};

use errors::{FileId, Reporter, WithError};

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[derive(Debug)]
pub(crate) struct ResolverDataCollector<DB> {
    db: DB,
    table: FileTable,
    reporter: Reporter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum State {
    Declared,
    Defined,
    Read,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Scopes {
    scopes: Vec<HashMap<hir::NameId, State>>,
    len: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct FunctionData {
    scopes: Vec<HashMap<hir::NameId, util::Span<State>>>,
    locals: HashMap<util::Span<hir::NameId>, usize>,
}

impl FunctionData {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            ..Self::default()
        }
    }

    pub(crate) fn peek(&self) -> usize {
        self.scopes.len() - 1
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FileTable {
    /// FUNCTION NAME TO FUNCTION SPAN
    symbol_level: HashMap<hir::NameId, hir::Span>,
    symbol_exports: HashSet<hir::NameId>,
    function_data: HashMap<hir::FunctionId, FunctionData>,
}

impl FileTable {
    pub fn new() -> Self {
        Self {
            symbol_exports: HashSet::default(),
            symbol_level: HashMap::default(),
            function_data: HashMap::default(),
        }
    }

    pub(crate) fn peek(&self, name: hir::FunctionId) -> usize {
        self.function_data[&name].peek()
    }

    pub(crate) fn function_data(&self, name: hir::FunctionId) -> &FunctionData {
        &self.function_data[&name]
    }

    pub(crate) fn function_data_mut(&mut self, name: hir::FunctionId) -> &mut FunctionData {
        self.function_data.get_mut(&name).unwrap()
    }

    pub fn contains(&self, id: hir::NameId) -> bool {
        self.symbol_level.get(&id).is_some()
    }

    pub fn has_export(&self, id: &hir::NameId) -> bool {
        self.symbol_exports.get(id).is_some()
    }

    pub fn get_span(&self, id: hir::NameId) -> hir::Span {
        self.symbol_level[&id]
    }

    pub(crate) fn insert_name(&mut self, name: hir::NameId, span: hir::Span, exported: bool) {
        self.symbol_level.insert(name, span);

        if exported {
            self.symbol_exports.insert(name);
        }
    }

    pub(crate) fn insert_function_data(&mut self, id: hir::FunctionId, data: FunctionData) {
        self.function_data.insert(id, data);
    }
}

impl<'a, DB> ResolverDataCollector<&'a DB>
where
    DB: HirDatabase,
{
    fn table(self) -> FileTable {
        self.table
    }

    fn reporter(self) -> Reporter {
        self.reporter
    }

    fn begin_scope(&mut self, function: hir::FunctionId) {
        let data = self.table.function_data_mut(function);
        data.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self, function: hir::FunctionId) {
        let data = self.table.function_data_mut(function);

        let scopes = data.scopes.pop().unwrap();

        for (name, state) in &scopes {
            println!("{:?}", name);
            if state.item == State::Defined {
                let msg = format!("Unused variable `{}`", self.db.lookup_intern_name(*name));
                self.reporter
                    .warn(msg, "", (state.start().to_usize(), state.end().to_usize()))
            }
        }
    }

    fn peek(&self, function_name: hir::FunctionId) -> usize {
        self.table.peek(function_name)
    }

    pub(crate) fn declare(&mut self, function: hir::FunctionId, name: util::Span<hir::NameId>) {
        let index = self.peek(function);

        if self.table.function_data(function).scopes[index].contains_key(&name.item) {
            let msg = format!(
                "The name `{}` already exists in this scope.",
                self.db.lookup_intern_name(name.item)
            );

            self.reporter
                .warn(msg, "", (name.start().to_usize(), name.end().to_usize()))
        }

        self.table.function_data_mut(function).scopes[index].insert(
            name.item,
            util::Span::new(State::Declared, name.start(), name.end()),
        );
    }

    pub(crate) fn define(&mut self, function: hir::FunctionId, name: util::Span<hir::NameId>) {
        let index = self.peek(function);

        self.table.function_data_mut(function).scopes[index].insert(
            name.item,
            util::Span::new(State::Defined, name.start(), name.end()),
        );
    }

    pub(crate) fn not_resolved(
        &mut self,
        function: hir::FunctionId,
        name: &util::Span<hir::NameId>,
    ) -> bool {
        let index = self.peek(function);

        if let Some(state) = self.table.function_data(function).scopes[index].get(&name.item) {
            return state.item == State::Declared;
        } else {
            false
        }
    }

    pub(crate) fn define_pattern(
        &mut self,
        function: hir::FunctionId,
        ast_map: &hir::FunctionAstMap,
        pat_id: &hir::PatId,
    ) {
        let pat = ast_map.pat(pat_id);

        match &pat {
            hir::Pattern::Bind { name } => self.define(function, *name),
            hir::Pattern::Tuple(pats) => pats
                .iter()
                .for_each(|pat| self.define_pattern(function, ast_map, &pat.item)),
            hir::Pattern::Literal(_) => {}
            hir::Pattern::Placeholder => {}
        }
    }

    pub(crate) fn insert_function_data(&mut self, id: hir::FunctionId, data: FunctionData) {
        self.table.insert_function_data(id, data);
    }

    pub(crate) fn insert_top_level(
        &mut self,
        file: FileId,
        id: hir::FunctionId,
        name: hir::NameId,
        exported: bool,
        span: hir::Span,
    ) {
        if self.table.contains(name) {
            let name = self
                .db
                .lookup_intern_name(self.db.lower_function(file, id).name.item);

            self.reporter.error(
                format!("The name `{}` is defined multiple times", name),
                "",
                (span.start().to_usize(), span.end().to_usize()),
            );
        } else {
            self.table.insert_name(name, span, exported);
            self.insert_function_data(id, FunctionData::new());
        }
    }

    pub(crate) fn resolve_statement(
        &mut self,
        function: hir::FunctionId,
        ast_map: &hir::FunctionAstMap,
        stmt: &hir::StmtId,
    ) {
        let stmt = ast_map.stmt(stmt);

        match stmt {
            hir::Stmt::Let { pat, initializer } => {
                self.resolve_pattern(function, ast_map, &pat.item);

                if let Some(init) = initializer {
                    self.resolve_expression(function, ast_map, init)
                }

                self.define_pattern(function, ast_map, &pat.item);
            }

            hir::Stmt::Expr(expr) => self.resolve_expression(function, ast_map, expr),
        }
    }

    pub(crate) fn resolve_expression(
        &mut self,
        function: hir::FunctionId,
        ast_map: &hir::FunctionAstMap,
        expr_id: &hir::ExprId,
    ) {
        let expr = ast_map.expr(expr_id);

        match expr {
            hir::Expr::Array(exprs) => exprs
                .iter()
                .for_each(|id| self.resolve_expression(function, ast_map, id)),
            hir::Expr::Binary {
                ref lhs, ref rhs, ..
            } => {
                self.resolve_expression(function, ast_map, lhs);
                self.resolve_expression(function, ast_map, rhs)
            }
            hir::Expr::Block(block_id) => {
                let block = ast_map.block(block_id);

                self.begin_scope(function);

                block
                    .0
                    .iter()
                    .for_each(|id| self.resolve_statement(function, ast_map, id));

                self.end_scope(function)
            }

            hir::Expr::Break | hir::Expr::Continue => {}
            hir::Expr::Call { callee, args } => {
                self.resolve_expression(function, ast_map, callee);

                args.iter()
                    .for_each(|id| self.resolve_expression(function, ast_map, id))
            }

            hir::Expr::Cast { expr, .. } => self.resolve_expression(function, ast_map, expr),
            hir::Expr::If {
                cond,
                then_branch,
                else_branch,
            } => {
                self.resolve_expression(function, ast_map, cond);
                self.resolve_expression(function, ast_map, then_branch);

                if let Some(else_branch) = else_branch {
                    self.resolve_expression(function, ast_map, else_branch);
                }
            }

            hir::Expr::Ident(name) => {
                let span = ast_map.expr_span(expr_id);
                if self.not_resolved(function, name) {
                    let msg = format!(
                        "Cannot read local name `{}` in its own initializer.",
                        self.db.lookup_intern_name(name.item)
                    );

                    self.reporter
                        .error(msg, "", (span.start().to_usize(), span.end().to_usize()))
                }

                self.resolve_local(function, name, true, span);
            }
            hir::Expr::Index { base, index } => {
                self.resolve_expression(function, ast_map, base);
                self.resolve_expression(function, ast_map, index);
            }
            hir::Expr::While { cond, body } => {
                self.resolve_expression(function, ast_map, cond);

                let block = ast_map.block(body);

                self.begin_scope(function);

                block
                    .0
                    .iter()
                    .for_each(|id| self.resolve_statement(function, ast_map, id));

                self.end_scope(function);
            }

            hir::Expr::Literal(_) => {}
            hir::Expr::Paren(id) => self.resolve_expression(function, ast_map, id),
            hir::Expr::Tuple(exprs) => exprs
                .iter()
                .for_each(|id| self.resolve_expression(function, ast_map, id)),
            hir::Expr::Unary { expr, .. } => self.resolve_expression(function, ast_map, expr),
            hir::Expr::Return(expr) => {
                if let Some(expr) = expr {
                    self.resolve_expression(function, ast_map, expr)
                }
            }

            hir::Expr::Match { expr, arms } => {
                self.resolve_expression(function, ast_map, expr);
                arms.iter().for_each(|arm| {
                    arm.pats
                        .iter()
                        .for_each(|pat_id| self.resolve_pattern(function, ast_map, &pat_id.item));
                    self.resolve_expression(function, ast_map, &arm.expr)
                })
            }
        }
    }

    pub(crate) fn resolve_local(
        &mut self,
        function: hir::FunctionId,
        name: &util::Span<hir::NameId>,
        is_read: bool,
        span: hir::Span,
    ) {
        let data = self.table.function_data_mut(function);
        let max_depth = data.scopes.len();

        for i in 0..max_depth {
            if data.scopes[max_depth - i - 1].contains_key(&name.item) {
                if is_read {
                    if let Some(state) = data.scopes[max_depth - i - 1].get_mut(&name.item) {
                        *state = util::Span::new(State::Read, name.start(), name.end())
                    }
                }

                return;
            }
        } // check for ident name in function/local scope

        if !self.table.contains(name.item) {
            //  check for external import global level
            let msg = format!(
                "Use of undefined variable `{}`",
                self.db.lookup_intern_name(name.item)
            );

            self.reporter
                .error(msg, "", (span.start().to_usize(), span.end().to_usize()))
        }
    }

    pub(crate) fn resolve_pattern(
        &mut self,
        function: hir::FunctionId,
        ast_map: &hir::FunctionAstMap,
        pat_id: &hir::PatId,
    ) {
        let pat = ast_map.pat(pat_id);

        match &pat {
            hir::Pattern::Bind { name } => self.declare(function, *name),
            hir::Pattern::Tuple(pats) => pats
                .iter()
                .for_each(|pat| self.resolve_pattern(function, ast_map, &pat.item)),
            hir::Pattern::Literal(_) => {}
            hir::Pattern::Placeholder => {}
        }
    }
}

pub fn resolve_exports_query(db: &impl HirDatabase, file: FileId) -> WithError<Arc<FileTable>> {
    let program = db.lower(file)?;
    let reporter = Reporter::new(file);
    let mut collector = ResolverDataCollector {
        db,
        reporter: reporter.clone(),
        table: FileTable::new(),
    };

    for function in &program.functions {
        collector.insert_top_level(
            file,
            function.id,
            function.name.item,
            function.exported,
            function.span,
        )
    }

    if reporter.has_errors() {
        Err(reporter.finish())
    } else {
        Ok(Arc::new(collector.table()))
    }
}

pub fn resolve_source_file_query(db: &impl HirDatabase, file: FileId) -> WithError<()> {
    let program = db.lower(file)?;

    println!("{:#?}", program.functions);
    let reporter = Reporter::new(file);

    let mut collector = ResolverDataCollector {
        db,
        reporter,
        table: FileTable::new(),
    };

    // collect the top level definitions first so we can
    // use forward declarations

    for import in &program.imports {
        db.resolve_import(file, import.id)?;
    }

    for function in &program.functions {
        collector.insert_top_level(
            file,
            function.id,
            function.name.item,
            function.exported,
            function.span,
        )
    }

    for function in &program.functions {
        let ast_map = function.map();

        if function.body().is_none() {
            continue;
        }

        for statement in function.body().as_ref().unwrap() {
            collector.resolve_statement(function.id, ast_map, statement)
        }
    }

    let reporter = collector.reporter();

    if reporter.has_errors() {
        Err(reporter.finish())
    } else {
        Ok(())
    }
}
