use lexer::get_tokens;
use parser::{get_ast_tree};

fn main() {
	let source = "
    interface Test {
		optional const_property: String { \"Constant    string variable!\" };
    
		enum Status {
			TEST: untest;
			UNTEST: test;
		};
	};
    ";
	let tokens = get_tokens(source);
	let tree = get_ast_tree(tokens);

	for node in tree.nodes.iter() {
		println!("Tree node: {:#?}", node);
	}
}
