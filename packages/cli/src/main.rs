use lexer::{get_tokens};
use parser::{parse_tokens};

fn main() {
    let source = "
    interface User { 
        required id -> String;
        optional username -> String;

        interface Book {
            required id -> String;
            optional title -> String;
            optional published -> Boolean;
            required author -> String;

            enum Status {
                SOLD;
                STOCK;
            };
        };
    };
    ";
    let tokens = get_tokens(source.clone());
    let tree = parse_tokens(tokens);

    for node in tree.nodes.iter() {
        println!("Tree node: {:#?}", node);
    };
}