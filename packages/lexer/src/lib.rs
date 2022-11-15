use logos::Logos;
use tokens::{TokenDeclaration, TokenType};

pub mod helpers;
pub mod tokens;

pub fn get_tokens(source: &str) -> Vec<TokenDeclaration> {
    let mut declarations = Vec::<TokenDeclaration>::new();
    let mut lexer = TokenType::lexer(source);

    let mut current_token = lexer.next();

    while current_token != None {
        match current_token.unwrap() {
            TokenType::Text => {
                declarations.push(TokenDeclaration::new(TokenType::Text, Option::Some(lexer.slice().to_string()), lexer.span()));
            }
            TokenType::Error => { /* Ignoring... */ }
            token_type => {
                declarations.push(TokenDeclaration::new(token_type, Option::None, lexer.span()));
            }
        }

        // Updating current token information
        current_token = lexer.next();
    };

    declarations
}