use syntax::T;

use crate::parser::{Parser, Restrictions};

use crate::SyntaxKind::*;

use crate::parser::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_return_expr(&mut self) {
        self.start_node(RETURN_EXPR);

        self.expect(T![return]);

        if self.current() == T![;] {
            self.finish_node();
            return;
        }

        self.parse_expression(Precedence::Assignment, Restrictions::default());

        self.finish_node()
    }
}

#[cfg(test)]
mod tests {
    test_parser! {parse_return_expr,"fn main() {return 1+1;}"}
    test_parser! {parse_empty_return_expr,"fn main() {return;}"}
}
