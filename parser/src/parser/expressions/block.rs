use syntax::T;

use crate::parser::{pratt::Precedence, Parser};

use crate::{Span, SyntaxKind::*, Token};

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Span<Token>>,
{
    pub(crate) fn parse_block(&mut self) {
        self.start_node(BLOCK);

        self.expect(T!["{"], "Expected `{`");

        while !self.at(EOF) && !self.at(T!["}"]) {
            match self.current() {
                T![let] => self.parse_let_expr(),
                T![if] => self.parse_if_expr(),
                T![do] => self.parse_do_expr(),
                T![while] => self.parse_while_expr(),
                T![return] => self.parse_return_expr(),
                T![break] => self.parse_break_expr(),
                T![continue] => self.parse_continue_expr(),
                T![for] => self.parse_for_expr(),
                WHITESPACE => {
                    while self.at(WHITESPACE) {
                        self.bump();
                    }
                    continue;
                }
                _ => {
                    self.start_node(EXPR_STMT);
                    self.parse_expression(Precedence::Assignment);
                    self.finish_node();
                }
            }

            if !self.at(T!["}"]) && !self.expected(T![;]) {
                break;
            }
        }

        self.expect(T!["}"], "Expected `}`");

        self.finish_node()
    }
}

#[cfg(test)]
mod test {}
