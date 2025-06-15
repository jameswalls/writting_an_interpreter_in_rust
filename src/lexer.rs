use crate::tokens::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self { 
            input,
            position: 0,
            read_position: 0,
            ch: 0
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        self.ch = *self.input.as_bytes().get(self.read_position).unwrap_or(&0);
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            b'=' => Token::new(TokenType::ASSIGN, "=".to_string()),
            b';' => Token::new(TokenType::SEMICOLON, ";".to_string()),
            b'(' => Token::new(TokenType::LPAREN, "(".to_string()),
            b')' => Token::new(TokenType::RPAREN, ")".to_string()),
            b',' => Token::new(TokenType::COMMA, ",".to_string()),
            b'+' => Token::new(TokenType::PLUS, "+".to_string()),
            b'{' => Token::new(TokenType::LBRACE, "{".to_string()),
            b'}' => Token::new(TokenType::RBRACE, "}".to_string()),
            _ => Token::new(TokenType::EOF, "".to_string())
        };
        self.read_char();
        token
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

        let mut lexer = Lexer::new(input);

        expected.iter().for_each(|t| {
            let token = lexer.next_token();
            assert_eq!(t, &token)
        });
    }
}
