use std::ops::Range;

use lexer::tokens::{TokenDeclaration, TokenType};

use crate::helpers::{next_token, next_token_index};

use super::VariableType;

pub fn parse_string(
	tokens: &Vec<TokenDeclaration>,
	start_index: usize,
) -> (VariableType, Range<usize>) {
	// String declaration expected at start index
	if tokens.get(start_index).unwrap().token_type != TokenType::StringType {
		panic!(
			"String type expected, got {:?}",
			tokens.get(start_index).unwrap()
		);
	};

	// Right curly braces expected (if anything else - return variable type)
	if next_token(tokens, start_index, Option::None).token_type == TokenType::RightCurlyBraces {
		let mut value: Option<String> = Option::None;
		let mut end_index = start_index;

		// This String is a constant, so let's parse it!
		if next_token(tokens, start_index, Option::Some(2)).token_type == TokenType::Quotes {
			println!("Parse multiline string!!!");

			// Parsing multi-line const string
			let (string, range) = parse_multiline_string(
				tokens,
				next_token_index(tokens, start_index, Option::Some(2)),
			);

			// Updating value and end_index
			value = Option::Some(string);
			end_index = range.end;
		} else {
			// Text type expected
			if tokens.get(start_index + 2).unwrap().token_type != TokenType::Text {
				panic!(
					"String type's constant value expected, got {:?}",
					tokens.get(start_index + 2).unwrap()
				);
			}

			// Updating value and end_index
			value = tokens.get(start_index + 2).unwrap().value.clone();
			end_index = start_index + 2;
		}

		// Left curly braces expected at the end
		if next_token(tokens, end_index, Option::None).token_type != TokenType::LeftCurlyBraces {
			panic!(
				"Left curly braces expected, got {:?}",
				tokens.get(end_index + 1)
			);
		}

		// Returning parsed variable
		(
			VariableType::String(value),
			Range {
				start: start_index,
				end: next_token_index(tokens, end_index, Option::Some(1)),
			},
		)
	} else {
		// Returning this variable type
		(
			VariableType::String(Option::None),
			Range {
				start: start_index,
				end: start_index,
			},
		)
	}
}

pub fn parse_multiline_string(
	tokens: &Vec<TokenDeclaration>,
	start_index: usize,
) -> (String, Range<usize>) {
	// Start token is always a Quote
	if tokens.get(start_index).unwrap().token_type != TokenType::Quotes {
		panic!("Quote expected, got {:?}", tokens.get(start_index).unwrap());
	};

	let mut result: Vec<String> = Vec::new();
	let mut end_index: Option<usize> = Option::None;

	// Let's get EVERYTHING until end quotes or until token end
	for (index, token) in tokens.iter().enumerate() {
		if index < start_index + 1 {
			continue;
		};

		// Adding to strings vector or exiting for loop (if it's an quote)
		match token.token_type {
			TokenType::Quotes => {
				// Updating end_index and breaking for loop.
				end_index = Option::Some(index);
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

	if end_index == Option::None {
		panic!("String type's constant value end quotes expected, got nothing");
	};

	// Returning result
	(
		result.join(""),
		Range {
			start: start_index,
			end: end_index.unwrap(),
		},
	)
}
