use core::ops::Range;
use entities::{
	enumerate::{parse_enum, Enum},
	interface::{parse_interface, Interface},
};
use helpers::create_linear_numbers_array;
use lexer::{
	get_tokens,
	tokens::{TokenDeclaration, TokenType},
};

pub mod entities;
pub mod helpers;
pub mod types;

#[derive(Debug)]
pub enum Entity {
	Interface(Interface),
	Enum(Enum),
}

#[derive(Debug)]
pub struct Node {
	pub range: Range<usize>,
	pub nodes: Vec<Node>,
	pub entity: Entity,
}

pub struct Parser {
	pub tree: Tree,
	pub tokens: Vec<TokenDeclaration>,
	pub source: String,
}

impl Parser {
	pub fn default(source: String) -> Parser {
		Self {
			tree: Tree::default(),
			tokens: get_tokens(source.clone().as_str()),
			source,
		}
	}
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
		}

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

pub fn parse_source(source: String) -> Parser {
	let mut parser = Parser::default(source);

	for (index, token) in parser.tokens.iter().enumerate() {
		// Checking if we already parsed token on this index
		if !parser.tree.parsed_indicies.contains(&index.clone()) {
			match token.token_type.clone() {
				TokenType::InterfaceDeclaration => {
					let node = parse_interface(&parser, index);
					parser.tree.add_node(node);
				}
				TokenType::EnumerateDeclaration => {
					let node = parse_enum(&parser, index);
					parser.tree.add_node(node);
				}
				token_type => {
					// Error
					panic!("{:?} is not an top-level keyword.", token_type);
				}
			};
		};
	}

	parser
}
