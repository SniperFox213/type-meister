use core::ops::Range;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum TokenType {
	//
	// Interface Declaration
	//
	#[token("interface")]
	InterfaceDeclaration,

	#[token("{")]
	RightCurlyBraces,

	#[token("}")]
	LeftCurlyBraces,

	//
	// Interface Variables
	//

	// Interface variable declarations and modifiers
	#[token("required")]
	RequiredModifier,

	#[token("optional")]
	OptionalModifier,

	#[token("->")]
	VariableConnection,

	// Built-in variables types
	#[token("String")]
	StringType,

	#[token("Int")]
	IntegerType,

	#[token("Boolean")]
	BooleanType,

	#[token(";")]
	Semicolon,

	//
	// Enumerates
	//
	#[token("enum")]
	EnumerateDeclaration,

	// Text
	#[regex("[a-zA-Z]+")]
	Text,

	// Logos requires one token variant to handle errors,
	// it can be named anything you wish.
	#[error]
	// We can also use this variant to define whitespace,
	// or any other matches we wish to skip.
	#[regex(r"[ \t\n\f]+")]
	Error,
}

#[derive(Debug, Clone)]
pub struct TokenDeclaration {
	pub token_type: TokenType,
	pub value: Option<String>,
	pub span: Range<usize>,
}
