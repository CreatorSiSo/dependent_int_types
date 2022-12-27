use crate::{lexer::Token, types::Int};
use logos::Lexer;

pub fn parse_file(lexer: &mut Lexer<Token>) -> Vec<Stmt> {
	vec![Stmt::Expr(parse_expr(lexer))]
}

pub fn parse_expr(lexer: &mut Lexer<Token>) -> Expr {
	match lexer.next().unwrap() {
		Token::Number(int) => Expr::Literal(int),
		v => panic!("{v:?}"),
	}
}

#[derive(Debug)]
pub enum Stmt {
	Const,
	Expr(Expr),
}

#[derive(Debug)]
pub enum Expr {
	Literal(Int),
}
