mod version;
mod lexer;
mod node;
mod parser;
mod codegen;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        version::show_version();
        version::show_usage();
    } else {
        let filepath = args[1].clone();
        // test
        // tokenize
        let tokens = lexer::run(filepath.clone());
        for tok in &tokens {
            println!("{:?}", tok);
        }
        // parse
        let nodes = parser::run(filepath.clone(), tokens);
        for node in &nodes {
            println!("{:?}", node);
        }

        unsafe {
            let mut codegen = codegen::Codegen::new("mod");
            codegen.gen();
        }
    }
}
