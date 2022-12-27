use clap::Parser;
use codespan::files::SimpleFile;
use depint::{lexer::Token, parser::DiagnosticHandler, DepIntParser};
use logos::Logos;
use std::fs::read_to_string;

/// Interpreter for a tiny language used to test out dependent integer types for Rym
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Location of the file to execute
	#[arg(short, long)]
	file: String,
}

fn main() {
	let args = Args::parse();
	let source = match read_to_string(&args.file) {
		Ok(input) => input,
		Err(err) => {
			println!("error: {err}");
			return;
		}
	};

	let mut parser = DepIntParser {
		handler: DiagnosticHandler::new(SimpleFile::new(args.file, &source)),
		lexer: Token::lexer(dbg!(&source)),
	};
	let ast = parser.parse_file();
	dbg!(ast);
}
