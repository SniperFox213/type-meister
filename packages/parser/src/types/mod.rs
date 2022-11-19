use core::ops::Range;
use lexer::tokens::{TokenDeclaration, TokenType};

use self::string::parse_string;

pub mod string;

#[derive(Debug, PartialEq)]
pub enum VariableType {
	String(Option<String>),
}

pub fn parse_variable_type(tokens: &Vec<TokenDeclaration>, start_index: usize) -> (VariableType, Range<usize>) {
	match tokens.get(start_index).unwrap().token_type {
		TokenType::StringType => parse_string(tokens, start_index),
		_ => {
			panic!("Could not parse variable type at {:?}", tokens.get(start_index).unwrap());
		}
	}
}
