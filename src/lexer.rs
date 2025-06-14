use crate::tokens::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize, // current position in the input (points to the char)
    readPosition: usize, // current reading position (after current char)
    ch: u8, // current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Self {
        todo!()
    }

    pub fn next_token(&self) -> Token {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "\
=+(){},;".to_string();

        let expected: Vec<Token> = vec![
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::PLUS, "+".to_string()),
            Token::new(TokenType::LPAREN, "(".to_string()),
            Token::new(TokenType::RPAREN, ")".to_string()),
            Token::new(TokenType::LBRACE, "{".to_string()),
            Token::new(TokenType::RBRACE, "}".to_string()),
            Token::new(TokenType::COMMA, ",".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::EOF, "".to_string()),
        ];

        let lexer = Lexer::new(input);

        expected.iter().for_each(|t| {
            let token = lexer.next_token();
            assert_eq!(t, &token)
        });
    }
}
