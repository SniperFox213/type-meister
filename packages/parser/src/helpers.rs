use core::ops::Range;

use lexer::tokens::{TokenDeclaration, TokenType};

use crate::errors::ParserError;

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

pub fn next_token_with_index(
	tokens: &Vec<TokenDeclaration>,
	start_index: usize,
	mut skip: Option<usize>,
) -> Result<(usize, TokenDeclaration), ParserError> {
	if skip == Option::None {
		skip = Option::Some(1);
	};

	let mut skipped = 0;
	let mut found_token: Option<(usize, TokenDeclaration)> = Option::None;

	// Iterating from start and trying to find next non-whitespace token
	for (index, token) in tokens.iter().enumerate() {
		// We need to inclide our start token, so that it'll be skipped
		if index < start_index {
			continue;
		};

		match token.token_type.clone() {
			TokenType::Whitespace => { /* Ignoring */ }
			_ => {
				// Checking if we need to skip this token or to return it
				if skipped < skip.unwrap() {
					skipped += 1;
				} else {
					found_token = Option::Some((index, token.clone()));
					break;
				};
			}
		};
	}

	if found_token.is_none() {
		// Todo ignore this panic?
		return Err(ParserError::empty())
	}

	Ok(found_token.unwrap())
}

pub fn next_token(
	tokens: &Vec<TokenDeclaration>,
	start_index: usize,
	mut skip: Option<usize>,
) -> TokenDeclaration {
	match next_token_with_index(tokens, start_index, skip) {
		Ok(token) => {
			token.1
		},
		Err(_) => {
			panic!("To implement");
		}
	}
}

pub fn next_token_index(
	tokens: &Vec<TokenDeclaration>,
	start_index: usize,
	mut skip: Option<usize>,
) -> usize {
	match next_token_with_index(tokens, start_index, skip) {
		Ok(token) => {
			token.0
		},
		Err(_) => {
			panic!("To implement");
		}
	}
}
