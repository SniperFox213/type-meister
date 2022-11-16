use parser::parse_source;

fn main() {
	let source = "
    interface Test { 
		required required_property: String;
		optional const_property: String { \"This is a constant\" };
    };
    ";
	let parser = parse_source(source.to_string());

	for node in parser.tree.nodes.iter() {
		println!("Tree node: {:#?}", node);
	}
}
