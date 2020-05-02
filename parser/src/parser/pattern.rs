use crate::parser::{Parser, Precedence, Restrictions};
use syntax::T;

use crate::SyntaxKind::*;

impl<'a> Parser<'a> {
    pub(crate) fn parse_pattern(&mut self, allow_literal: bool) {
        match self.current() {
            T!["("] => self.parse_tuple_pattern(allow_literal),
            IDENT => self.parse_binding_pattern(),
            T![self] => self.bump(),
            T![_] => self.parse_placeholder_pattern(),
            e => {
                if allow_literal {
                    self.start_node(LITERAL_PAT);
                    self.parse_expression(Precedence::Primary, Restrictions::default());
                    self.finish_node()
                } else {
                    self.error(
                        "Expected a literal pattern",
                        format!(
                            "Expected a literal pattern instead found `{}` {:?}",
                            self.current_string(),
                            e
                        ),
                    )
                }
            }
        }
    }

    fn parse_tuple_pattern(&mut self, allow_literal: bool) {
        self.start_node(TUPLE_PAT);

        self.bump(); // Eat the `(`

        while !self.at(EOF) && !self.at(T![")"]) {
            self.parse_pattern(allow_literal);
            if !self.at(T![")"]) && !self.expected(T![,]) {
                break;
            }
        }

        self.expect(T![")"]);
        self.finish_node();
    }

    fn parse_placeholder_pattern(&mut self) {
        self.start_node(PLACEHOLDER_PAT);
        self.expect(T![_]);
        self.finish_node();
    }

    fn parse_binding_pattern(&mut self) {
        self.start_node(BIND_PAT);
        self.ident();
        self.finish_node();
    }
}

#[cfg(test)]
mod test {
    test_parser! {parse_placeholder_pattern,"fn main(_:i32) {}"}

    test_parser! {parse_tuple_pattern,"fn main((x,y):i32) {}"}

    test_parser! {parse_binding_pattern,"fn main(x:i32) {}"}

    test_parser! {parse_nested_tuple_pattern,"fn main((x,(y,_)):i32) {}"}
}
