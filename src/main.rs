use std::fs;

mod error;
mod interpreter;
mod parser;

fn main() {
    let contents = fs::read_to_string("resources/test/day5.intcode").unwrap();
    let code = parser::parse(&contents).unwrap();
    let results = interpreter::run(code);
    println!("{:#?}", results);
}
