use std::{env, fs};

mod scanner;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = args.get(1).expect("must provide a file");
    let input = fs::read_to_string(fname).expect("file does not exist");
    let tokens = scanner::scan(input);
    println!("{:#?}", tokens);
    let ast = parser::parse(tokens);
    println!("{:#?}", ast);
}
