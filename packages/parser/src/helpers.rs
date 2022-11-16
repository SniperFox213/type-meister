use core::ops::Range;

use lexer::tokens::{TokenDeclaration, TokenType};

use crate::Parser;

pub fn create_linear_numbers_array(mut from: usize, to: usize) -> Vec<usize> {
	let mut numbers = Vec::<usize>::new();

	while from <= to {
		numbers.push(from);
		from += 1;
	}

	numbers
}

pub fn get_slice_from_source(source: &str, span: Range<usize>) -> String {
	let mut chars = Vec::<char>::new();

	for (i, char) in source.clone().chars().into_iter().enumerate() {
		if i >= span.start && i < span.end {
			chars.push(char);
		};
	}

	chars.into_iter().collect()
}

pub fn parse_multiline_string(parser: &Parser, start_index: usize) -> (String, Range<usize>) {
	// Start token is always a Quote
	if parser.tokens.get(start_index).unwrap().token_type != TokenType::Quotes {
		panic!(
			"Quote expected, got {:?}",
			parser.tokens.get(start_index).unwrap()
		);
	};

	let mut span: Range<usize> = Range {
		start: parser.tokens.get(start_index + 1).unwrap().span.start,
		end: 0,
	};
	let mut end_index: Option<usize> = Option::None;

	// Let's get EVERYTHING until end quotes or until token end
	for (index, token) in parser.tokens.iter().enumerate() {
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
					Some(_) => {
						// Updating our span
						span = Range {
							start: span.start,
							end: token.span.end,
						};
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
		get_slice_from_source(parser.source.clone().as_str(), span),
		Range {
			start: start_index,
			end: end_index.unwrap(),
		},
	)
}
