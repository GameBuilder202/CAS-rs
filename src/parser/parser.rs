use lalrpop_util::lalrpop_mod;

use crate::expr::nodes::Expr;

lalrpop_mod!(grammar, "/src/parser/grammar.rs");

pub fn parse(string: &str) -> Expr {
    let parser = grammar::ExprParser::new();
    let res = parser.parse(string).unwrap();
    res
}
