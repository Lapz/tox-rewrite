use syntax::T;

use crate::parser::Parser;

use crate::SyntaxKind::*;

impl<'a> Parser<'a> {
    pub(crate) fn parse_break_expr(&mut self) {
        self.start_node(BREAK_EXPR);

        self.expect(T![break]);

        self.finish_node()
    }
}

#[cfg(test)]
mod tests {
    test_parser! {parse_break_expr,"fn main() {break;}"}
}
