use std::collections::HashMap;

#[derive(Debug)]
pub enum Token {
    Contract,
    Identifier(u32),
    Is,
    Comma,
    EndLine,
    NoToken
}

pub trait CharacterVector {
    fn to_chars(&'static self) -> Vec<char>;
}

impl CharacterVector for str {
    fn to_chars(&'static self) -> Vec<char> {
        self.chars().collect::<Vec<char>>()
    }
}

impl Token {
    fn from_alphanumeric_string(token: String, symbols: &mut HashMap<String, u32>, table_index: &mut u32) -> Token {
        if token.len() == 0 {
            return Token::NoToken;
        } else if token == "contract" {
            return Token::Contract;
        } else if token == "is" {
            return Token::Is;
        }
        let symbol = symbols.get(&token);
        return match symbol {
            Some(id) => Token::Identifier(*id),
            None => {
                symbols.insert(token, *table_index);
                *table_index += 1;
                Token::Identifier(*table_index - 1)
            }
        }
    }
}

pub fn next_token(input: &Vec<char>, cur: &mut usize, symbols: &mut HashMap<String, u32>, table_index: &mut u32) -> Token {
    let mut result = Token::NoToken;
    let mut collected = String::new();
    while *cur < input.len() {
        if collected.len() == 0 {
            if input[*cur] == ',' {
                *cur += 1;
                return Token::Comma;
            } else if input[*cur] == '\n' || input[*cur] == ';' {
                *cur += 1;
                return Token::EndLine;
            } else if input[*cur].is_alphanumeric() {
                collected.push(input[*cur]);
            } 
        } else {
            if !input[*cur].is_alphanumeric() {
                break;
            } else {
                collected.push(input[*cur]);
            }
        }
        *cur += 1;
    }
    Token::from_alphanumeric_string(collected, symbols, table_index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn comma_test1() { 
        match next_token(&",".to_chars(), &mut 0, &mut HashMap::new(), &mut 1) {
            Token::Comma => (),
            actual => panic!("Expected: {:?} | Actual {:?}", Token::Comma, actual)
        }
    }
    
    #[test]
    fn contract_test1() {
        match next_token(&"contract".to_chars(), &mut 0, &mut HashMap::new(), &mut 1) {
            Token::Contract => (),
            actual => panic!("Expected: {:?} | Actual {:?}", Token::Contract, actual)
        }
    }

    #[test]
    fn is_test1() {
        match next_token(&"is".to_chars(), &mut 0, &mut HashMap::new(), &mut 1) {
            Token::Is => (),
            actual => panic!("Expected: {:?} | Actual {:?}", Token::Is, actual)
        }
    }

    #[test]
    fn identifier_test1() {
        match next_token(&"linearize".to_chars(), &mut 0, &mut HashMap::new(), &mut 1) {
            Token::Identifier(id) => assert_eq!(id, 1),
            actual => panic!("Expected: {:?} | Actual {:?}", Token::Identifier(1), actual)
        }
    }

    #[test]
    fn no_token_test1() {
        match next_token(&"".to_chars(), &mut 0, &mut HashMap::new(), &mut 1) {
            Token::NoToken => (),
            actual => panic!("Expected: {:?} | Actual {:?}", Token::NoToken, actual)
        }
    }
}
