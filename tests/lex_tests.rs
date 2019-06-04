extern crate linearizer;

#[cfg(test)]
mod lex_tests {
    use linearizer::lex::{ next_token, CharacterVector, Token };
    use std::collections::HashMap;

    #[test]
    fn lex_integration_test1() {
        let cur = &mut 0;
        let cur_idx = &mut 1;
        let input = "   contract A; contract B13as is A".to_chars();
        let symbol_table = &mut HashMap::new();
        match next_token(&input, cur, symbol_table, cur_idx) {
            Token::Contract => (),
            actual => panic!("Expected: {:?} | Actual {:?}", Token::Contract, actual)
        }
        match next_token(&input, cur, symbol_table, cur_idx) {
            Token::Identifier(id) => assert_eq!(id, 1),
            actual => panic!("Expected: {:?} | Actual {:?}", Token::Identifier(1), actual)
        }
        match next_token(&input, cur, symbol_table, cur_idx) {
            Token::EndLine => (),
            actual => panic!("Expected: {:?} | Actual {:?}", Token::EndLine, actual)
        }
        match next_token(&input, cur, symbol_table, cur_idx) {
            Token::Contract => (),
            actual => panic!("Expected: {:?} | Actual {:?}", Token::Contract, actual)
        }
        match next_token(&input, cur, symbol_table, cur_idx) {
            Token::Identifier(id) => assert_eq!(id, 2),
            actual => panic!("Expected: {:?} | Actual {:?}", Token::Identifier(2), actual)
        }
        match next_token(&input, cur, symbol_table, cur_idx) {
            Token::Is => (),
            actual => panic!("Expected: {:?} | Actual {:?}", Token::Is, actual)
        } 
        match next_token(&input, cur, symbol_table, cur_idx) {
            Token::Identifier(id) => assert_eq!(id, 1),
            actual => panic!("Expected: {:?} | Actual {:?}", Token::Identifier(1), actual)
        }
    }
}
