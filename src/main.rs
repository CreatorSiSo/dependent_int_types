use clap::Parser;
use depint::lexer::Token;
use depint::parser::parse_file;
use logos::Logos;
// use codespan_reporting::diagnostic::Diagnostic;
// use codespan_reporting::term::emit;
// use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::fs::read_to_string;

/// Interpreter for a tiny language used to test out dependent integer types for Rym
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Location of the file to execute
	#[arg(short, long)]
	file: String,
}

fn main() {
	let args = Args::parse();
	// let diagnostics: Vec<Diagnostic<_>> = vec![];
	// let mut writer = StandardStream::stderr(ColorChoice::Auto);

	let input = match read_to_string(args.file) {
		Ok(input) => input,
		Err(err) => {
			println!("error: {err}");
			return;
		}
	};

	let mut lexer = Token::lexer(dbg!(&input));
	let ast = parse_file(&mut lexer);
	dbg!(ast);
}
