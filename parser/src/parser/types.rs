use crate::parser::{Parser, Precedence, Restrictions};
use crate::SyntaxKind::*;
use syntax::T;

impl<'a> Parser<'a> {
    pub(crate) fn parse_type(&mut self) {
        match self.current() {
            IDENT | T![self] => self.parse_ident_type(),
            T!["["] => self.parse_array_type(),
            T!["("] => self.parse_paren_type(),
            T![fn] => self.parse_fn_type(),
            _ => self.error(
                "Expected a type parameter",
                format!(
                    "Expected a type parameter but instead found `{}`",
                    self.current_string()
                ),
            ),
        };
    }

    fn parse_ident_type(&mut self) {
        self.start_node(IDENT_TYPE);

        if self.matches(vec![IDENT, T![self]]) {
            self.bump();
        } else {
            self.error(
                "Expected an identifier or `void`",
                format!(
                    "Expected an identifier or `void` but instead found `{}`",
                    self.current_string()
                ),
            )
        }

        if self.at(T![<]) {
            self.parse_type_args();
        }

        self.finish_node();
    }

    fn parse_array_type(&mut self) {
        self.start_node(ARRAY_TYPE);
        self.bump(); //Eat `[`

        self.parse_type();

        if self.at(T![;]) {
            self.bump();
            self.parse_expression(Precedence::Primary, Restrictions::default())
        }

        self.expect(T!["]"]);

        self.finish_node();
    }

    fn parse_paren_type(&mut self) {
        self.start_node(PAREN_TYPE);
        self.bump(); //Eat `(`

        while !self.at(EOF) && !self.at(T![")"]) {
            self.parse_type();
            if !self.at(T![")"]) && !self.expected(T![,]) {
                break;
            }
        }

        self.expect(T![")"]);

        self.finish_node();
    }

    fn parse_fn_type(&mut self) {
        self.start_node(FN_TYPE);

        self.expect(T![fn]);

        self.expect(T!["("]);

        while !self.at(EOF) && !self.at(T![")"]) {
            self.parse_type();
            if !self.at(T![")"]) && !self.expected(T![,]) {
                break;
            }
        }

        self.expect(T![")"]);

        if self.at(T![->]) {
            self.start_node(RET_TYPE);
            self.bump();
            self.parse_type();
            self.finish_node();
        }

        self.finish_node();
    }
}

#[cfg(test)]
mod test {
    test_parser! {parse_fn_type,"fn main(_:fn(i32,i32) -> i32) {}"}
    test_parser! {parse_fn_tuple_type,"fn main(_:fn((i32,i32)) -> i32) {}"}
    test_parser! {parse_ident_type,"fn main(x:i32) {}"}
    test_parser! {parse_array_type,"fn main(x:[i32]) {}"}
    test_parser! {parse_array_tuple_type,"fn main(x:[(i32,i32)]) {}"}
    test_parser! {parse_tuple_type,"fn main(x:(i32,i32)) {}"}
}
