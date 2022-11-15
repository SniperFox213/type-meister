use lexer::tokens::{TokenDeclaration, TokenType};
use core::ops::Range;
use crate::{Node, helpers::create_linear_numbers_array};

#[derive(Debug)]
pub struct Interface {
  pub name: String,
  pub variables: Vec<InterfaceVariable>,
}

#[derive(Debug)]

pub struct InterfaceVariable {
  pub name: String,
  pub variable_type: String,
  pub is_required: bool,
}

pub fn parse_interface(tokens: &Vec<TokenDeclaration>, start_index: usize) -> Node {
  // Interface information
  let mut name: Option<String> = Option::None;
  let mut variables = Vec::<InterfaceVariable>::new();

  // 
  let mut is_inside_interface = false;
  let mut parsed_indecies = Vec::<usize>::new();
  let mut end_index: Option<usize> = Option::None;

  for (index, token) in tokens.iter().enumerate() {
      if index < start_index { continue; };
      if index <= 0 { continue; };

      if (is_inside_interface) {
          // Skipping parsed content
          if parsed_indecies.contains(&index) { continue; }; 

          match token.token_type.clone() {
              TokenType::OptionalModifier | TokenType::RequiredModifier => {
                let (variable, range) = parse_variable(tokens, index);
                  
                // Adding this variable to interface's variable list
                variables.push(variable);

                // Adding this range to parsed_indecies
                for parsed_index in create_linear_numbers_array(range.start, range.end) {
                  parsed_indecies.push(parsed_index);
                }
              },
              TokenType::InterfaceDeclaration => {
                  println!("Parsing interface...");
              },
              TokenType::LeftCurlyBraces => {
                // Interface is parsed. Checking if we have a semicolon after
                // this brace
                if (tokens.len() >= index + 1) && (tokens.get(index + 1).unwrap().token_type == TokenType::Semicolon) {
                  // Ending
                  end_index = Option::Some(index + 1);
                  break;
                };

                // Semicolon expected...
                panic!("Semicolon expected...");
              },
              _ => {
                  panic!("Variable or interface declaration expected, got: {:?}", token);
              },
          };
      } else {
          if index == start_index + 1 {
              // Interface name expected
              if token.token_type != TokenType::Text {
                  panic!("Interface name expected");
              } else {
                  name = token.value.clone();
              };
          } else {
              // Right curcly braces expected
              if token.token_type != TokenType::RightCurlyBraces {
                  panic!("Right curly braces expected");
              } else {
                  is_inside_interface = true;
              };
          };
      };
  };

  if end_index == Option::None { panic!("No end index"); };

  Node {
      span: Range {
        start: tokens.get(start_index).unwrap().span.start, 
        end: tokens.get(end_index.unwrap()).unwrap().span.end, 
      },
      nodes: Vec::new(),
      entity: Interface {
        name: name.unwrap(),
        variables 
      }
  }
}

fn parse_variable(tokens: &Vec<TokenDeclaration>, start_index: usize) -> (InterfaceVariable, Range<usize>) {
  // Variable options
  let mut is_required: Option<bool> = Option::None;
  let mut name: Option<(usize, String)> = Option::None;
  let mut variable_type: Option<String> = Option::None;

  let mut end_index: Option<usize> = Option::None;

  // 
  for (index, token) in tokens.iter().enumerate() {
    if index < start_index { continue; };

    // Modifiers or variable name expected (if variable name is not present)
    if name == Option::None {
      match token.token_type.clone() {
        // Requir<ance> (???) modifiers
        TokenType::RequiredModifier | TokenType::OptionalModifier => {
          if (is_required == Option::None) {
            is_required = Option::Some(token.token_type == TokenType::RequiredModifier);
          } else {
            if (is_required.unwrap() && token.token_type == TokenType::OptionalModifier) || (!is_required.unwrap() && token.token_type == TokenType::RequiredModifier) {
              panic!("This variable is already {}. It can not be both", if is_required.unwrap() { "required" } else { "optional" });
            } else {
              panic!("This variable is already {}.", if is_required.unwrap() { "required" } else { "optional" });
            }
          };
        },
        TokenType::Text => {
          name = Option::Some((index, token.value.clone().unwrap()));
        },
        _ => {
          panic!("Variable modifiers or variable name expected, got: {:?}", token);
        }
      }
    } else {
      // Name is present.
      if index == name.clone().unwrap().0 + 1 {
        // VariableConnection expected
        if token.token_type != TokenType::VariableConnection {
          panic!("-> expected, got {:?}", token);
        }
      } else {
        if variable_type == Option::None {
          // Variable type expected
          match token.token_type.clone() {
            TokenType::StringType => {
              variable_type = Option::Some("String".to_string());
            },
            _ => {
              panic!("Variable type expected, got {:?}", token);
            }
          };
        } else {
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
  };

  if end_index == Option::None { panic!("No end index"); };

  (InterfaceVariable {
    name: name.unwrap().1,
    variable_type: variable_type.unwrap(),
    is_required: is_required.unwrap()
  }, Range { start: start_index, end: end_index.unwrap() })
}