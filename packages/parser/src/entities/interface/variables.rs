use std::ops::Range;

use lexer::tokens::{TokenDeclaration, TokenType};

use crate::{types::{VariableType, parse_variable_type}, helpers::{next_token_index, create_linear_numbers_array}};

#[derive(Debug)]

pub struct InterfaceVariable {
	pub name: String,
	pub variable_type: VariableType,
	pub is_required: bool,
}

pub fn parse_variable(
	tokens: &Vec<TokenDeclaration>,
	start_index: usize,
) -> (InterfaceVariable, Range<usize>) {
	println!(
		"Parsing variable at {:?} {:?}",
		start_index,
		tokens.get(start_index).clone().unwrap()
	);

	// Variable options
	let mut is_required: Option<bool> = Option::None;
	let mut name: Option<(usize, String)> = Option::None;
	let mut variable_type: Option<VariableType> = Option::None;

	let mut end_index: Option<usize> = Option::None;
	let mut parsed_indicies: Vec<usize> = Vec::new();

	for (index, token) in tokens.iter().enumerate() {
		if index <= 0 {
			continue;
		}
		if index < start_index {
			continue;
		};
		if parsed_indicies.contains(&index) {
			continue;
		};

		// Modifiers or variable name expected (if variable name is not present)
		if name == Option::None {
			match token.token_type.clone() {
				// Requir<ance> (???) modifiers
				TokenType::RequiredModifier | TokenType::OptionalModifier => {
					if (is_required == Option::None) {
						is_required = Option::Some(token.token_type == TokenType::RequiredModifier);
					} else {
						if (is_required.unwrap() && token.token_type == TokenType::OptionalModifier)
							|| (!is_required.unwrap()
								&& token.token_type == TokenType::RequiredModifier)
						{
							panic!(
								"This variable is already {}. It can not be both",
								if is_required.unwrap() {
									"required"
								} else {
									"optional"
								}
							);
						} else {
							panic!(
								"This variable is already {}.",
								if is_required.unwrap() {
									"required"
								} else {
									"optional"
								}
							);
						}
					};
				}
				TokenType::Text => {
					name = Option::Some((index, token.value.clone().unwrap()));
				}
				TokenType::Whitespace => { /* Ignoring */ }
				_ => {
					panic!(
						"Variable modifiers or variable name expected, got: {:?}",
						token
					);
				}
			}
		} else {
			// Name is present.
			if index == next_token_index(tokens, name.clone().unwrap().0, Option::None) {
				// VariableConnection expected
				if token.token_type != TokenType::VariableConnection {
					panic!(": expected, got {:?}", token);
				}
			} else {
				if variable_type == Option::None {
					// Variable type expected
					match token.token_type.clone() {
						TokenType::StringType => {
							let (variable, range) = parse_variable_type(tokens, index);
							variable_type = Option::Some(variable);

							// Adding this range to our parsed_indicies array
							for parsed_index in create_linear_numbers_array(range.start, range.end)
							{
								parsed_indicies.push(parsed_index);
							}
						}
						TokenType::Whitespace => { /* Ignoring */ }
						_ => {
							panic!("Variable type expected, got {:?}", token);
						}
					};
				} else {
					// Ignoring whitespaces
					if token.token_type == TokenType::Whitespace {
						continue;
					};

					// Semicolon expected
					if token.token_type != TokenType::Semicolon {
						panic!("Semicolon expected, got {:?}", token);
					} else {
						// Variable parsing ended
						end_index = Option::Some(index);
						break;
					};
				}
			};
		};
	}

	if end_index == Option::None {
		panic!("No end index");
	};

	(
		InterfaceVariable {
			name: name.unwrap().1,
			variable_type: variable_type.unwrap(),
			is_required: is_required.unwrap(),
		},
		Range {
			start: start_index,
			end: end_index.unwrap(),
		},
	)
}
