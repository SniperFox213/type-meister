use std::ops::Range;

use lexer::tokens::{TokenDeclaration, TokenType, self};

use crate::helpers::{next_token, next_token_index, next_token_with_index};

use super::VariableType;

// 
// String type parsing
// 
// Example:
// optional var_name: String;
// 						^ This function parses this section
// optional const_string: String { Const };
// optional const_string: String { "Multi-line const!!" };
// 
// Structure:
// StringType
// | StringType RightCurlyBraces Text LeftCurlyBraces
// | StringType RightCurlyBraces Quotes [any token]+ Quotes LeftCurlyBraces
// 
// P.S. Semicolon is checked in parse_variable (entities/interface/variables.rs) function
pub fn parse_string(
	tokens: &Vec<TokenDeclaration>,
	start_index: usize,
) -> (VariableType, Range<usize>) {
	// Parsing info
	let mut current_index = start_index;
	let mut is_multi_line = false;
	let mut value: Option<String> = Option::None;

	// 
	// String declaration expected at start index
	{
		let token = match tokens.get(current_index) {
			Some(token) => token,
			None => {
				panic!("Expected StringType token, got nothing");
			},
		};

		if token.token_type != TokenType::StringType {
			panic!("Expected StringType, got {:?}", token);
		};
	}

	// 
	// Right curly braces expected for const variable
	// 					 or
	// Semicolon

	{
		let (index, token) = match next_token_with_index(tokens, current_index, Option::None) {
			Ok(token) => token,
			Err(_) => {
				panic!("Semicolon or RightCurlyBraces expected, got nothing");
			},
		};

		if token.token_type == TokenType::Semicolon {
			println!("Returning our variable!");
			// Returning our variable
			return (
				VariableType::String(Option::None),
				Range {
					start: start_index,
					end: start_index,
				},
			)
		} else if token.token_type == TokenType::RightCurlyBraces {
			/* Ignoring (we will parse multi-line string later) */
		} else {
			panic!("Semicolon or RightCurlyBraces expected, got {:?}", token);
		}

		// Updating current index
		current_index = index;
	}

	// *This string is a multi-line string
	//
	// Quotes or Text expected

	{
		let (index, token) = match next_token_with_index(tokens, current_index, Option::None) {
			Ok(token) => token,
			Err(_) => {
				panic!("Quotes or Text expected, got nothing");
			}
		};

		if token.token_type == TokenType::Text {
			// Updating value
			value = token.value;

			// Updating current_index
			current_index = index;
		} else if token.token_type == TokenType::Quotes {
			// Parsing multi-line string
			is_multi_line = true;

			let (string, range) = parse_multiline_string(tokens, next_token_index(tokens, current_index, Option::None));
			value = Option::Some(string);

			// Updating current_index
			current_index = range.end;
		}
	}

	// 
	// LeftCurlyBraces 
	// | Quotes LeftCurlyBraces
	// 
	// expected (depending on whatever this is a multi-line string or no)
	
	// P.S.
	// Quotes was already parsed in parse_multiline_string function

	// LeftCurlyBraces expected
	{
		let (index, token) = match next_token_with_index(tokens, current_index, Option::None) {
			Ok(token) => token,
			Err(_) => {
				panic!("LeftCurlyBraces expected, got nothing");
			},
		};

		if token.token_type != TokenType::LeftCurlyBraces {
			panic!("LeftCurlyBraces expected, got {:?}", token);
		};

		current_index = index;
	}

	// Returning our const string variable
	(
		VariableType::String(value),
		Range {
			start: start_index,
			end: current_index,
		}
	)
}

pub fn parse_multiline_string(
	tokens: &Vec<TokenDeclaration>,
	start_index: usize,
) -> (String, Range<usize>) {
	// Parser information
	let mut current_index = start_index;
	let mut result: Vec<String> = Vec::new();

	// Start token is always a Quote
	{
		let token = match tokens.get(current_index) {
			Some(token) => token,
			None => {
				panic!("Quotes expected, got nothing");
			},
		};

		if token.token_type != TokenType::Quotes {
			panic!("Quotes expected, got {:?}", token);
		};
	}

	// Let's get EVERYTHING until end quotes or until token end
	for (index, token) in tokens.iter().enumerate() {
		if index < current_index + 1 {
			continue;
		};

		// Adding to strings vector or exiting for loop (if it's an quote)
		match token.token_type {
			TokenType::Quotes => {
				// Updating current_index and breaking for loop.
				current_index = index;
				break;
			}
			_ => {
				// Adding this TokenDeclaration's value
				match token.value.clone() {
					Some(value) => {
						result.push(value);
					}
					None => { /* Ignoring */ }
				}
			}
		};
	}

	// Returning result
	(
		result.join(""),
		Range {
			start: start_index,
			end: current_index,
		},
	)
}
