use crate::{
	helpers::{create_linear_numbers_array, next_token_index},
	types::{parse_variable_type, VariableType},
	Entity, Node, Tree,
};
use core::ops::Range;
use lexer::tokens::{TokenDeclaration, TokenType};

use super::enumerate::parse_enum;

#[derive(Debug)]
pub struct Interface {
	pub name: String,
	pub variables: Vec<InterfaceVariable>,
}

#[derive(Debug)]

pub struct InterfaceVariable {
	pub name: String,
	pub variable_type: VariableType,
	pub is_required: bool,
}

pub fn parse_interface(tokens: &Vec<TokenDeclaration>, start_index: usize) -> Node {
	// Interface information
	let mut name: Option<String> = Option::None;
	let mut nodes = Vec::<Node>::new();
	let mut variables = Vec::<InterfaceVariable>::new();

	let mut is_inside_interface = false;
	let mut parsed_indicies = Vec::<usize>::new();
	let mut end_index: Option<usize> = Option::None;

	for (index, token) in tokens.iter().enumerate() {
		if index <= 0 {
			continue;
		};
		if index - 1 < start_index {
			continue;
		};

		if (is_inside_interface) {
			// Skipping parsed content
			if parsed_indicies.contains(&index) {
				continue;
			};

			match token.token_type.clone() {
				TokenType::OptionalModifier | TokenType::RequiredModifier => {
					let (variable, range) = parse_variable(tokens, index);

					// Adding this variable to interface's variable list
					variables.push(variable);

					// Adding this range to parsed_indicies
					for parsed_index in create_linear_numbers_array(range.start, range.end) {
						parsed_indicies.push(parsed_index);
					}
				},
				TokenType::EnumerateDeclaration => {
					// Parsing sub-enumerate
					let sub_enumerate = parse_enum(tokens, index);

					// Adding this range to parsed_indicies
					for parsed_index in create_linear_numbers_array(
						sub_enumerate.range.start,
						sub_enumerate.range.end,
					) {
						parsed_indicies.push(parsed_index);
					}

					// Adding this sub_enumerate to our nodes variable
					nodes.push(sub_enumerate);
				},
				TokenType::InterfaceDeclaration => {
					let sub_interface = parse_interface(tokens, index);

					// Adding this range to parsed_indicies
					for parsed_index in create_linear_numbers_array(
						sub_interface.range.start,
						sub_interface.range.end,
					) {
						parsed_indicies.push(parsed_index);
					}

					// Adding this sub_interface to our nodes variable
					nodes.push(sub_interface);
				},
				TokenType::LeftCurlyBraces => {
					// Interface is parsed. Checking if we have a semicolon after
					// this brace
					if (tokens.len() >= index + 1)
						&& (tokens.get(index + 1).unwrap().token_type
							== TokenType::Semicolon)
					{
						// Ending
						end_index = Option::Some(index + 1);
						break;
					};

					// Semicolon expected...
					panic!("Semicolon expected...");
				},
				TokenType::Whitespace => { /* Ignoring */ },
				_ => {
					panic!(
						"Variable or interface declaration expected, got: {:?}",
						token
					);
				}
			};
		} else {
			if index == next_token_index(tokens, start_index, Option::None) {
				// Interface name expected
				if token.token_type != TokenType::Text {
					panic!("Interface name expected, got {:?}", token);
				} else {
					name = token.value.clone();
				};
			} else {
				// Ignoring whitespace
				if token.token_type == TokenType::Whitespace { continue; };

				// Right curcly braces expected
				if token.token_type != TokenType::RightCurlyBraces {
					panic!("Right curly braces expected, got {:?}", token);
				} else {
					is_inside_interface = true;
				};
			};
		};
	}

	if end_index == Option::None {
		panic!("No end index");
	};

	Node {
		range: Range {
			start: start_index,
			end: end_index.unwrap(),
		},
		nodes,
		entity: Entity::Interface(Interface {
			name: name.unwrap(),
			variables,
		}),
	}
}

fn parse_variable(tokens: &Vec<TokenDeclaration>, start_index: usize) -> (InterfaceVariable, Range<usize>) {
	println!("Parsing variable at {:?} {:?}", start_index, tokens.get(start_index).clone().unwrap());

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
							};
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
