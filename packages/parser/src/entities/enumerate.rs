use lexer::tokens::TokenDeclaration;
use crate::Node;

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

pub fn parse_enum(tokens: &Vec<TokenDeclaration>, start_index: usize) -> Node {
  
}