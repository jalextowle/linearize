use std::env;
use std::fs;
use linearizer::lex;
use linearizer::linearize;

fn main() {
    let name = env::args().nth(1).expect("Usage");
    let input = fs::read_to_string(name).expect("Unable to open input file");
}
