use core::ops::Range;
use lexer::tokens::{TokenDeclaration, TokenType};

use crate::Parser;

use self::string::parse_string;

pub mod string;

#[derive(Debug, PartialEq)]
pub enum VariableType {
	String(Option<String>),
}

pub fn parse_variable_type(parser: &Parser, start_index: usize) -> (VariableType, Range<usize>) {
	let token = parser.tokens.get(start_index).unwrap();

	match token.token_type {
		TokenType::StringType => parse_string(parser, start_index),
		_ => {
			panic!("Could not parse variable type at {:?}", token);
		}
	}
}
