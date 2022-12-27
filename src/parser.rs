use std::fmt::Debug;
use std::ops::Range;

use crate::{lexer::Token, types::Int};
use codespan::diagnostic::{Diagnostic, Label, LabelStyle};
use codespan::files::SimpleFile;
use codespan::term::termcolor::ColorChoice;
use codespan::term::{self, termcolor::StandardStream};
use logos::Lexer;

pub struct Parser<'s> {
	pub handler: DiagnosticHandler<'s>,
	pub lexer: Lexer<'s, Token>,
}

impl Parser<'_> {
	pub fn parse_file(&mut self) -> Vec<Stmt> {
		let mut stmts = vec![];
		while let Some(peek) = self.lexer.clone().next() {
			let result = match peek {
				Token::Const => self
					.parse_const()
					.map(|constant| stmts.push(Stmt::Const(constant))),
				_ => self.parse_expr().map(|expr| stmts.push(Stmt::Expr(expr))),
			};

			if result.is_none() {
				// TODO Consume until Newline
			}
		}
		stmts
	}

	pub fn parse_const(&mut self) -> Option<Const> {
		let start = self.lexer.span().start;
		debug_assert_eq!(self.lexer.next(), Some(Token::Const));

		let name = if let Some(Token::Ident(name)) = self.lexer.next() {
			name
		} else {
			self.handler.emit(
				Diagnostic::error()
					.with_message("Expected constant | expr")
					.with_labels(vec![Label::new(LabelStyle::Primary, (), self.lexer.span())]),
			);
			"<unknown>".into()
		};

		debug_assert_eq!(self.lexer.next(), Some(Token::Eq));

		let Some(expr) = self.parse_expr() else {
			todo!();
			// return None;
		};

		Some(Const {
			name,
			expr,
			type_: None,
			span: start..self.lexer.span().end,
		})
	}

	pub fn parse_expr(&mut self) -> Option<Expr> {
		let Token::Number(int) = self.expect(|token| matches!(token, Token::Number(_)))? else {
			unreachable!()
		};
		Some(Expr::Literal(int))
	}

	fn expect<F>(&mut self, predicate: F) -> Option<Token>
	where
		F: Fn(&Token) -> bool,
	{
		match self.lexer.next() {
			Some(got) if predicate(&got) => Some(got),
			Some(token) => {
				self.handler.emit(
					Diagnostic::error()
						.with_message(format!("Expected __TODO__ got `{token:?}`"))
						.with_labels(vec![Label::new(LabelStyle::Primary, (), self.lexer.span())]),
				);
				None
			}
			None => {
				self.handler.emit(
					Diagnostic::error()
						.with_message(format!("Expected __TODO__"))
						.with_labels(vec![Label::new(LabelStyle::Primary, (), self.lexer.span())]),
				);
				None
			}
		}
	}
}

#[derive(Debug)]
pub enum Stmt {
	Const(Const),
	Expr(Expr),
}

#[derive(Debug)]
pub struct Const {
	pub name: String,
	pub expr: Expr,
	pub type_: Option<()>,
	pub span: Range<usize>,
}

#[derive(Debug)]
pub enum Expr {
	Literal(Int),
}

pub struct DiagnosticHandler<'s> {
	pub file: SimpleFile<String, &'s str>,
	pub writer: StandardStream,
}

/// Emits formatted source errors and other output.
impl<'s> DiagnosticHandler<'s> {
	pub fn new(file: SimpleFile<String, &'s str>) -> Self {
		Self {
			file,
			writer: StandardStream::stderr(ColorChoice::Auto),
		}
	}

	pub fn emit(&self, diagnostic: Diagnostic<()>) {
		term::emit(
			&mut self.writer.lock(),
			&Default::default(),
			&self.file,
			&diagnostic,
		)
		.unwrap();
	}
}
