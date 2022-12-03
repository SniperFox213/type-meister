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

	#[token(":")]
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
	#[regex("[a-zA-Z_0-9]+")]
	Text,

	// Multi-line text helpers
	#[token("\"")]
	Quotes,

	// Whitespace
	#[regex(r"[ \t\n\f]+")]
	Whitespace,

	// Enything else
	#[error]
	Error,
}

#[derive(Debug, Clone)]
pub struct TokenDeclaration {
	pub token_type: TokenType,
	pub value: Option<String>,
	pub span: Range<usize>,
}
