use std::fs;

mod error;
mod interpreter;
mod parser;
mod transpiler;

fn main() {
    let contents = fs::read_to_string("resources/test/day5.intcode").unwrap();
    let code = parser::parse(&contents).unwrap();
    let results = transpiler::transpile(code, vec![1]).unwrap();
    println!("{}", results);
}
