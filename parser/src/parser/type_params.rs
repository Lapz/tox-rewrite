use crate::parser::Parser;
use crate::SyntaxKind::*;
use syntax::T;

impl<'a> Parser<'a> {
    pub(crate) fn parse_type_params(&mut self, allow_types: bool) {
        self.start_node(TYPE_PARAM_LIST);
        self.bump();

        while !self.at(EOF) && !self.at(T![>]) {
            if allow_types {
                self.start_node(TYPE_PARAM);
                self.parse_type();
                self.finish_node();
            } else {
                self.type_param();
            }

            if !self.at(T![>]) && !self.expected(T![,]) {
                break;
            }
        }

        self.expect(T![>]);
        self.finish_node()
    }

    fn type_param(&mut self) {
        self.start_node(TYPE_PARAM);
        self.ident();
        self.finish_node();
    }
}
