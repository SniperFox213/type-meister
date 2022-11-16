use logos::Logos;
use tokens::{TokenDeclaration, TokenType};

pub mod tokens;

pub fn get_tokens(source: &str) -> Vec<TokenDeclaration> {
	let mut declarations = Vec::<TokenDeclaration>::new();
	let mut lexer = TokenType::lexer(source);

	let mut current_token = lexer.next();

	while current_token != None {
		declarations.push(TokenDeclaration {
			token_type: current_token.unwrap(),
			value: Option::Some(lexer.slice().to_string()),
			span: lexer.span(),
		});

		// Updating current token information
		current_token = lexer.next();
	}

	declarations
}
