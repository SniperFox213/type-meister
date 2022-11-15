use helpers::create_linear_numbers_array;
use lexer::tokens::{TokenDeclaration,TokenType};
use core::ops::Range;
use entities::{interface::{parse_interface, Interface}, enumerate::Enum};

pub mod helpers;
pub mod entities;

#[derive(Debug)]
enum Entity {
    Interface(Interface),
    Enum(Enum),
}

#[derive(Debug)]
pub struct Node {
    pub range: Range<usize>,
    pub nodes: Vec<Node>,
    pub entity: Entity,
}

pub struct Tree {
    pub nodes: Vec<Node>,
    pub parsed_indicies: Vec<usize>,
}

impl Tree {
    pub fn add_node(&mut self, node: Node) {
        let range = node.range.clone();
        
        // Adding token indecies to parsed_indicies vector
        for index in create_linear_numbers_array(range.start, range.end) {
            if !self.parsed_indicies.contains(&index.clone()) {
                self.parsed_indicies.push(index);
            };
        };

        // Pushing node to nodes array
        self.nodes.push(node);
    }

    pub fn default() -> Tree {
        Self {
            nodes: Vec::new(),
            parsed_indicies: Vec::new(),
        }
    }
}

pub fn parse_tokens(tokens: Vec<TokenDeclaration>) -> Tree {
    let mut tree = Tree::default();
    
    for (index, token) in tokens.iter().enumerate() {
        // Checking if we already parsed token on this index
        if !tree.parsed_indicies.contains(&index.clone()) {
            match token.token_type.clone() {
                TokenType::InterfaceDeclaration => {
                    let node = parse_interface(&tokens, index);
                    tree.add_node(node);
                }
                token_type => {
                    // Error
                    panic!("{:?} is not an top-level keyword.", token_type);
                }
            };
        };
    };

    tree
}