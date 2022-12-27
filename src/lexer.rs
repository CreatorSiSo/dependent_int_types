use crate::parser::Int;
use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
	#[token("+")]
	Plus,
	#[token("-")]
	Minus,

	#[regex("[0-9]+[0-9_]*", number)]
	Number(Int),

	#[token("const")]
	Const,
	#[regex("[_a-zA-Z]+")]
	Ident,

	#[token("\n")]
	Newline,

	#[error]
	#[regex(r"[ \t\f]+", logos::skip)] // Whitespace
	Error,
}

fn number(lexer: &mut Lexer<Token>) -> Option<Int> {
	let digits: String = lexer.slice().chars().filter(|c| c != &'_').collect();
	let value: u128 = digits.parse().unwrap();

	const U8_MAX: u128 = u8::MAX as u128;
	const U16_MAX: u128 = u16::MAX as u128;
	const U32_MAX: u128 = u32::MAX as u128;
	const U64_MAX: u128 = u64::MAX as u128;

	Some(match value {
		0..=U8_MAX => Int::U8(value as u8),
		0..=U16_MAX => Int::U16(value as u16),
		0..=U32_MAX => Int::U32(value as u32),
		0..=U64_MAX => Int::U64(value as u64),
		0..=u128::MAX => Int::U128(value as u128),
	})
}
