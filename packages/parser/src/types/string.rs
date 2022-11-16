use std::ops::Range;

use lexer::tokens::{TokenDeclaration, TokenType};

use crate::{helpers::parse_multiline_string, Parser};

use super::VariableType;

pub fn parse_string(parser: &Parser, start_index: usize) -> (VariableType, Range<usize>) {
    // String declaration expected at start index
    if parser.tokens.get(start_index).unwrap().token_type != TokenType::StringType {
        panic!("String type expected, got {:?}", parser.tokens.get(start_index).unwrap());
    };

    
    // Right curly braces expected (if anything else - return variable type)
    if (parser.tokens.get(start_index + 1).unwrap().token_type == TokenType::RightCurlyBraces) {
        let mut value: Option<String> = Option::None;
        let mut end_index = start_index;

        // This String is a constant, so let's parse it!
        if parser.tokens.get(start_index + 2).unwrap().token_type == TokenType::Quotes {
            // Parsing multi-line const string
            let (string, range) = parse_multiline_string(parser, start_index + 2);

            // Updating value and end_index
            value = Option::Some(string);
            end_index = range.end;
        } else {
            // Text type expected
            if parser.tokens.get(start_index + 2).unwrap().token_type != TokenType::Text {
                panic!("String type's constant value expected, got {:?}", parser.tokens.get(start_index + 2).unwrap());
            }

            // Updating value and end_index
            value = parser.tokens.get(start_index + 2).unwrap().value.clone();
            end_index = start_index + 2;
        }

        // Left curly braces expected at the end
        if parser.tokens.get(end_index + 1).unwrap().token_type != TokenType::LeftCurlyBraces {
            panic!("Left curly braces expected, got {:?}", parser.tokens.get(end_index + 1));
        }

        // Returning parsed variable
        (
            VariableType::String(value),
            Range {
                start: start_index,
                end: end_index + 1,
            }
        )
    } else {
        // Returning this variable type
        (
            VariableType::String(Option::None),
            Range {
                start: start_index,
                end: start_index,
            }
        )
    }
}