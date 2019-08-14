use crate::T;

use crate::parser::Parser;

use crate::{Span, SyntaxKind::*, Token};

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Span<Token>>,
{
    pub(crate) fn parse_class(&mut self, has_visibility: bool) {
        self.start_node(CLASS_DEF);

        if has_visibility {
            self.parse_visibility();
        }

        self.expect(CLASS_KW, "Expected `class`");

        self.ident();

        if self.is_ahead(|t| t == T![<]) {
            self.parse_type_params(false);
        }

        self.parse_class_body();

        self.finish_node()
    }

    fn parse_class_body(&mut self) {
        // self.start_node()
        self.expect(T!["{"], "Expected `{`");

        while !self.at(EOF) && !self.at(T!["}"]) {
            let has_visibility = self.has_visibility();

            if has_visibility {
                match self.peek() {
                    IDENT => self.parse_named_field(),
                    FN_KW => self.parse_function(has_visibility),
                    e => {
                        println!("{:?}", e);
                        self.error("Expected an identifier | `pub` | `fn` ")
                    }
                }
            } else {
                match self.current() {
                    IDENT => self.parse_named_field(),
                    FN_KW => self.parse_function(has_visibility),
                    e => {
                        println!("{:?}", e);
                        self.error("Expected an identifier | `pub` | `fn` ")
                    }
                }
            }

            // if !self.at(T!["}"]) && !self.expected(T![,]) {
            //     break;
            // }
        }

        self.expect(T!["}"], "Expected `}`");

        // self.finish_node();
    }

    fn parse_named_field(&mut self) {
        self.start_node(NAMED_FIELD_DEF);
        self.ident();
        self.expect(T![:], "Expected `:`");
        self.parse_type();
        self.expect(T![;], "Expected `;`");
        self.finish_node();
    }
}

#[cfg(test)]
mod test {
    test_parser! {parse_empty_class,"class Foo {}"}
    test_parser! {parse_class_generic,"class Result<T,E> {}"}
    test_parser! {parse_class_fields,"class Person { name:String; surname:String;}"}
    test_parser! {parse_class_fields_methods,"class Person { name:String; surname:String; fn hello(self) {}}"}
    test_parser! {parse_class_methods,"class Person { name:String; surname:String; fn new() -> Person {}}"}
}