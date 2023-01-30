use lexer::get_tokens;
use parser::get_ast_tree;

fn main() {
	let source = "
    interface Test {
		optional const_variable: String { \"Multi-line string!\" };
		enum TestEnum {
			Test: Test2;
			TestMulti: \"Multi-line enum!!!\";
		};
	};

	interface Test_3 {};
    ";
	let tokens = get_tokens(source);
	let tree = get_ast_tree(tokens);

	for node in tree.nodes.iter() {
		println!("Tree node: {:#?}", node);
	}
}
