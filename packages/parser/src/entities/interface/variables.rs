use std::ops::Range;

use lexer::tokens::{TokenDeclaration, TokenType};

use crate::{types::{VariableType, parse_variable_type}, helpers::{next_token_index, create_linear_numbers_array, next_token_with_index}};

#[derive(Debug)]

pub struct InterfaceVariable {
	pub name: String,
	pub variable_type: VariableType,
	pub is_required: bool,
}

//
// Interface variable
//
// Example:
// ...
// optional is_registered: String;
// ...
//
// Structure:
// 1. (OptionalModifier | RequiredModifier) Text VariableConnection (StringType | BooleanType | IntegerType | ...)
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
	let mut is_required: Option<bool>;
	let mut name: Option<String>;
	let mut variable_type: Option<VariableType> = Option::None;

	let mut current_index: usize = start_index;

	// 
	// First of - we need to determine if this
	// variable is optional or required.
	{
		let token = match tokens.get(current_index) {
			Some(token) => token,
			None => {
				panic!("Variable access modifier expected, gotnothing");
			}
		};

		// Optional Modifier
		if token.token_type == TokenType::OptionalModifier {
			is_required = Option::Some(false);
		// Required Modifier
		} else if token.token_type == TokenType::RequiredModifier {
			is_required = Option::Some(true);
		// Error
		} else {
			panic!("Variable modifier expected, got {:?}", token);
		};
	}

	//
	// Variable name
	// > Text
	{
		let (index, token) = match next_token_with_index(tokens, current_index, Option::None) {
			Ok(token) => token,
			Err(_) => {
				panic!("Variable name expected, got nothing");
			}
		};

		if token.token_type != TokenType::Text {
			panic!("Variable name expected, got {:?}", token);
		};

		// Updating variable's name
		name = token.value;

		// Updating current_index
		current_index = index;
	}

	//
	// VariableConnection
	{
		let (index, token) = match next_token_with_index(tokens, current_index, Option::None) {
			Ok(token) => token,
			Err(_) => {
				panic!("VariableConnection expected, got nothing");
			}
		};

		if token.token_type != TokenType::VariableConnection {
			panic!("Variable connection expected, got {:?}", token);
		};

		// Updating current_index
		current_index = index;
	}

	// 
	// And, finally, we have variable type
	{
		let (index, token) = match next_token_with_index(tokens, current_index, Option::None) {
			Ok(token) => token,
			Err(_) => {
				panic!("Variable type expected, got nothing");
			}
		};

		// Parsing variable type
		match token.token_type {
			TokenType::StringType => {
				let (variable, range) = parse_variable_type(tokens, index);
				variable_type = Option::Some(variable);

				// Updating current_index
				current_index = range.end;
			},
			TokenType::Whitespace => { /* Ignoring */ },
			_ => {
				panic!("Variable type expected, got {:?}", token);
			}
		};
	}

	//
	// Lastly, we expect semicolon
	{
		let (index, token) = match next_token_with_index(tokens, current_index, Option::None) {
			Ok(token) => token,
			Err(_) => {
				panic!("Semicolon expected, got nothing");
			}
		};

		if token.token_type != TokenType::Semicolon {
			panic!("Semicolon expected, got {:?}", token);
		};

		// Updating current_index
		current_index = index;
	}

	// Returning our variable information
	(
		InterfaceVariable {
			name: name.unwrap(),
			variable_type: variable_type.unwrap(),
			is_required: is_required.unwrap(),
		},
		Range {
			start: start_index,
			end: current_index + 1,
		},
	)
}
