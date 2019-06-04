use super::lex;
use std::collections::HashMap;

// FIXME: This should return a nested vector of strings
pub fn parse(input: &Vec<char>) -> Vec<Vec<u32>> {
    let cur = &mut 0;
    let cur_idx = &mut 0;
    let symbols = &mut HashMap::new();

    let mut bases = vec![];
    let mut extensions = vec![];

    while true {
        match lex::next_token(input, cur, symbols, cur_idx) {
            lex::Token::Contract => (),
            lex::Token::NoToken => break,
            _ => panic!("Invalid inheritance hierarchy")
        }
        let old = *cur_idx;
        match lex::next_token(input, cur, symbols, cur_idx) {
            lex::Token::Identifier(id) => {
                if old == *cur_idx {
                    panic!("Identifier defined twice");
                }
                bases.push(id);
            }
            _ => panic!("Invalid inheritance hierarchy")
        }
        let mut is = false;
        match lex::next_token(input, cur, symbols, cur_idx) {
            lex::Token::Is => is = true,
            lex::Token::EndLine => (),
            lex::Token::NoToken => break,
            _ => panic!("Invalid inheritance hierarchy")
        }
        if is {
            extensions.push(parse_contract_list(input, cur, symbols, cur_idx)); 
        }
    }
    linearize_hierarchy(bases, extensions)
}

// Parse a list of contracts
fn parse_contract_list(
    input: &Vec<char>, 
    cur: &mut usize, 
    symbols: &mut HashMap<String, u32>, 
    cur_idx: &mut u32
) -> Vec<u32> {
    let old = *cur_idx;
    let mut result = vec![];
    while true {
        match lex::next_token(input, cur, symbols, cur_idx) {
            lex::Token::Identifier(id) => result.push(id),
            _ => panic!("Invalid contract list")
        }
        match lex::next_token(input, cur, symbols, cur_idx) {
            lex::Token::Comma => (), 
            _ => break
        }
    }
    if old != *cur_idx {
        panic!("Identifier has not been defined");
    }
    result
}

fn linearize_hierarchy(bases: Vec<u32>, extensions: Vec<Vec<u32>>) -> Vec<Vec<u32>> { 
    let mut result = vec![];
    if bases.len() != extensions.len() {
        panic!("Invalid hierarchy");
    }
    result
}

fn linearize(front: u32, mut list: Vec<u32>) -> Vec<u32> {
    let merged = merge(&list);
    list.insert(0, front);
    list
}

fn merge(list: &Vec<u32>) -> Vec<u32> { vec![] }

#[cfg(test)]
mod tests {

}
