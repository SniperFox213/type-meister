use std::ops::Range;

use crate::{helpers::create_linear_numbers_array, Entity, Node, Parser};
use lexer::tokens::{TokenDeclaration, TokenType};

#[derive(Debug)]
pub struct Enum {
	pub name: String,
	pub variants: Vec<EnumVariant>,
}

#[derive(Debug)]
pub struct EnumVariant {
	pub name: String,
	pub value: Option<String>,
}

pub fn parse_enum(parser: &Parser, start_index: usize) -> Node {
	// Enumerate information
	let mut name: Option<String> = Option::None;
	let mut variants: Vec<EnumVariant> = Vec::new();

	let mut is_inside_enum = false;
	let mut parsed_indicies: Vec<usize> = Vec::new();
	let mut end_index: Option<usize> = Option::None;

	// Parsing
	for (index, token) in parser.tokens.iter().enumerate() {
		if index <= 0 {
			continue;
		};
		if index - 1 < start_index {
			continue;
		};

		if (is_inside_enum) {
			// Skipping parsed content
			if parsed_indicies.contains(&index) {
				continue;
			};

			match token.token_type.clone() {
				// Text expected (or left curly braces)
				TokenType::Text => {
					// Parsing enumerate variant...
					let (variant, range) = parse_variant(&parser.tokens, index);

					// Adding this range to parsed_indicies
					for parsed_index in create_linear_numbers_array(range.start, range.end) {
						parsed_indicies.push(parsed_index);
					}

					// Adding this variant to variants
					variants.push(variant);
				}
				TokenType::LeftCurlyBraces => {
					// Enum has ended, checking if we have a semicolon
					// after this brace
					if (parser.tokens.len() >= index + 1)
						&& (parser.tokens.get(index + 1).unwrap().token_type
							== TokenType::Semicolon)
					{
						end_index = Option::Some(index + 1);
						break;
					};

					panic!("Semicolon expected");
				}
				_ => {
					panic!("Enum variant name expected, got {:?}", token);
				}
			};
		} else {
			if index == start_index + 1 {
				// Enum name expected
				if token.token_type != TokenType::Text {
					panic!("Enum name expected, got {:?}", token);
				};

				name = token.value.clone();
			} else {
				// Right curly braces expected
				if token.token_type != TokenType::RightCurlyBraces {
					panic!("Right curly braces expected, got {:?}", token);
				} else {
					is_inside_enum = true;
				};
			};
		};
	}

	if end_index == Option::None {
		panic!("No end index");
	};

	// Returning new Enum node
	Node {
		range: Range {
			start: start_index,
			end: end_index.unwrap(),
		},
		nodes: Vec::new(),
		entity: Entity::Enum(Enum {
			name: name.unwrap(),
			variants,
		}),
	}
}

fn parse_variant(
	tokens: &Vec<TokenDeclaration>,
	start_index: usize,
) -> (EnumVariant, Range<usize>) {
	let mut name: Option<(usize, String)> = Option::None;
	let mut value: Option<String> = Option::None;

	let mut end_index: Option<usize> = Option::None;

	for (index, token) in tokens.iter().enumerate() {
		if index < start_index {
			continue;
		};

		if name == Option::None {
			// Name expected
			if token.token_type != TokenType::Text {
				panic!("Variant name expected, got {:?}", token);
			} else {
				// Updating variant's name
				name = Option::Some((index, token.value.clone().unwrap()));
			};
		} else {
			// Next token after name must be VariableConnection (or Semicolon). So let's
			// check this!
			if index == name.clone().unwrap().0 + 1 {
				if token.token_type == TokenType::Semicolon {
					// Ending
					end_index = Option::Some(index);
					break;
				};

				if token.token_type != TokenType::VariableConnection {
					panic!("Variable connection or Semicolon expected, got {:?}", token);
				};
			} else {
				if value == Option::None {
					// So here'll go variant value (Text again)
					if token.token_type != TokenType::Text {
						panic!("Enumerate value expected, got {:?}", token);
					};

					// Updating variant's value;
					value = token.value.clone();
				} else {
					// Semicolon expected
					if token.token_type != TokenType::Semicolon {
						panic!("Semicolon expected, got {:?}", token);
					};

					// Ending
					end_index = Option::Some(index);
					break;
				};
			};
		};
	}

	// Returning this variant
	(
		EnumVariant {
			name: name.unwrap().1,
			value: value,
		},
		Range {
			start: start_index,
			end: end_index.unwrap(),
		},
	)
}
