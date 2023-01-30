use std::ops::Range;

use crate::{
	helpers::{create_linear_numbers_array, next_token_index, next_token_with_index},
	types::{string::{parse_multiline_string}},
	Entity, Node, Tree,
};
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

//
// Enumeration
//
// Example:
//````
// enum Name {
//   VarName: VarValue;
// }
// ```
//
// Structure:
// 1. EnumerationDeclaration Text RightCurlyBraces
// 2. Text VariableConnection (Text | Quotes Text Quotes) Semicolon
// 3. LeftCurlyBraces Semicolon
pub fn parse_enum(
	tokens: &Vec<TokenDeclaration>, 
	start_index: usize
) -> Node {
	// Enum options
	let mut name: Option<String>;
	let mut variants: Vec<EnumVariant> = Vec::new();

	let mut current_index: usize = start_index;
	let mut parsed_indicies: Vec<usize> = Vec::new();

	//
	// EnumDeclaration
	{
		let token = match tokens.get(current_index) {
			Some(token) => token,
			None => {
				panic!("Enum declaration expected, got nothing");
			}
		};

		if token.token_type != TokenType::EnumerateDeclaration {
			panic!("Enum declaration expected, got {:?}", token);
		};
	};

	//
	// Text
	// as enum name
	{
		let (index, token) = match next_token_with_index(tokens, current_index, Option::None) {
			Ok(response) => response,
			Err(_) => {
				panic!("Enum name expected, got nothing");
			}
		};

		if token.token_type != TokenType::Text {
			panic!("Enum name expected, got {:?}", token);
		};

		name = token.value;
		current_index = index;
	};

	//
	// RightCurlyBraces
	{
		let (index, token) = match next_token_with_index(tokens, current_index, Option::None) {
			Ok(response) => response,
			Err(_) => {
				panic!("RightCurlyBraces expected, got nothing");
			}
		};

		if token.token_type != TokenType::RightCurlyBraces {
			panic!("RightCurlyBraces expected, got {:?}", token);
		};

		current_index = index;
	};

	//
	// Parsing all enum variants
	for (index, token) in tokens.iter().enumerate() {
		if index <= current_index {
			continue;
		};

		if parsed_indicies.contains(&index) {
			continue;
		};

		match token.token_type {
			TokenType::Text => {
				let (variant, range) = parse_variant(tokens, index);

				variants.push(variant);
				
				for parsed_index in create_linear_numbers_array(range.start, range.end) {
					parsed_indicies.push(parsed_index);
				};
			}
			TokenType::Whitespace => { /* Ignoring */ }
			TokenType::LeftCurlyBraces => {
				// Breaking from loop to end enum parsing
				current_index = index;
				break;
			}
			_ => {
				panic!("Enum name or LeftCurlyBraces expected, got {:?}", token);
			}
		};
	};

	//
	// Semicolon
	{
		let (index, token) = match next_token_with_index(tokens, current_index, Option::None) {
			Ok(response) => response,
			Err(_) => {
				panic!("Expected Semicolon, got nothing");
			}
		};

		if token.token_type != TokenType::Semicolon {
			panic!("Semicolon expected, got {:?}", token);	
		};

		current_index = index;
	};

	// Returning our enum
	Node {
		range: Range {
			start: start_index,
			end: current_index,
		},
		nodes: Vec::new(),
		entity: Entity::Enum(Enum {
			name: name.unwrap(),
			variants,
		})
	}
}

//
// Parse enumeration variant
//
// Structure:
// 1. Text VariableConnection (Text | Quotes Text Quotes)
fn parse_variant(
	tokens: &Vec<TokenDeclaration>,
	start_index: usize,
) -> (EnumVariant, Range<usize>) {
	let mut name: Option<String> = Option::None;
	let mut value: Option<String> = Option::None;

	let mut current_index = start_index;

	//
	// Text
	{
		let token = match tokens.get(current_index) {
			Some(token) => token,
			None => {
				panic!("Text expected, got nothing");
			}
		};

		if token.token_type != TokenType::Text {
			panic!("Text expected, got {:?}", token);
		};

		name = token.value.clone();
	};

	//
	// VariableConnection
	{
		let (index, token) = match next_token_with_index(tokens, current_index, Option::None) {
			Ok(response) => response,
			Err(_) => {
				panic!("VariableConnection expected, got nothing");
			}
		};

		if token.token_type != TokenType::VariableConnection {
			panic!("VariableConnection expected, got {:?}", token);
		};

		current_index = index;
	};

	//
	// (Text | Quotes Text Quotes)
	{
		let (index, token) = match next_token_with_index(tokens, current_index, Option::None) {
			Ok(response) => response,
			Err(_) => {
				panic!("Text or Quotes expected, got nothing");
			}
		};

		match token.token_type {
			TokenType::Text => {
				value = token.value;
				current_index = index;
			}
			TokenType::Quotes => {
				// Parsing multi-line string using string's type helper
				let (line_value, range) = parse_multiline_string(tokens, index);

				value = Option::Some(line_value);
				current_index = range.end;
			}
			_ => {
				panic!("Text or Quotes expected, got {:?}", token);
			}
		};
	};

	//
	// Semicolon expected
	{
		let (index, token) = match next_token_with_index(tokens, current_index, Option::None) {
			Ok(response) => response,
			Err(_) => {
				panic!("Semicolon expected, got nothing");
			}
		};

		if token.token_type != TokenType::Semicolon {
			panic!("Semicolon expected, got {:?}", token);
		};

		current_index = index;
	};

	// Returning our variant
	(
		EnumVariant {
			name: name.unwrap(),
			value: value,
		},
		Range {
			start: start_index,
			end: current_index,
		},
	)
}
