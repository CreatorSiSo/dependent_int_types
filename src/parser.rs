use crate::lexer::Token;
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

#[derive(Debug, PartialEq, Eq)]
pub enum Int {
	U8(u8),
	U16(u16),
	U32(u32),
	U64(u64),
	U128(u128),
	I8(i8),
	I16(i16),
	I32(i32),
	I64(i64),
	I128(i128),
}
