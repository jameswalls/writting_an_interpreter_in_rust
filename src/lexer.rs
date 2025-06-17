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
        self.skip_whitespace();
        // TODO: refactor so this returns (TokenType, literal) and call Token::new once afterwards
        let token = match self.ch {
            b'=' => Token::new(TokenType::ASSIGN, "=".to_string()),
            b';' => Token::new(TokenType::SEMICOLON, ";".to_string()),
            b'(' => Token::new(TokenType::LPAREN, "(".to_string()),
            b')' => Token::new(TokenType::RPAREN, ")".to_string()),
            b',' => Token::new(TokenType::COMMA, ",".to_string()),
            b'+' => Token::new(TokenType::PLUS, "+".to_string()),
            b'-' => Token::new(TokenType::MINUS, "-".to_string()),
            b'!' => Token::new(TokenType::BANG, "!".to_string()),
            b'*' => Token::new(TokenType::ASTERISK, "*".to_string()),
            b'/' => Token::new(TokenType::SLASH, "/".to_string()),
            b'<' => Token::new(TokenType::LT, "<".to_string()),
            b'>' => Token::new(TokenType::GT, ">".to_string()),
            b'{' => Token::new(TokenType::LBRACE, "{".to_string()),
            b'}' => Token::new(TokenType::RBRACE, "}".to_string()),
            0 => Token::new(TokenType::EOF, "".to_string()),
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = Token::lookup_indent(&literal);
                    return Token::new(token_type, literal)
                } else if is_digit(self.ch) {
                    let literal = self.read_number();
                    let token_type = TokenType::INT;
                    return Token::new(token_type, literal)
                }
                else {
                    Token::new(TokenType::ILLEGAL, "".to_string())
                }
            }
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }

        self.input[start..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let start = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }

        self.input[start..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}

fn is_digit(ch: u8) -> bool {
    ch.is_ascii_digit()
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

    #[test]
    fn test_next_token2() {
        let input = "\
let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);".to_string();

        let expected: Vec<Token> = vec![
            Token::new(TokenType::LET, "let".to_string()),
            Token::new(TokenType::IDENT, "five".to_string()),
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::INT, "5".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::LET, "let".to_string()),
            Token::new(TokenType::IDENT, "ten".to_string()),
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::INT, "10".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::LET, "let".to_string()),
            Token::new(TokenType::IDENT, "add".to_string()),
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::FUNCTION, "fn".to_string()),
            Token::new(TokenType::LPAREN, "(".to_string()),
            Token::new(TokenType::IDENT, "x".to_string()),
            Token::new(TokenType::COMMA, ",".to_string()),
            Token::new(TokenType::IDENT, "y".to_string()),
            Token::new(TokenType::RPAREN, ")".to_string()),
            Token::new(TokenType::LBRACE, "{".to_string()),
            Token::new(TokenType::IDENT, "x".to_string()),
            Token::new(TokenType::PLUS, "+".to_string()),
            Token::new(TokenType::IDENT, "y".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::RBRACE, "}".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::LET, "let".to_string()),
            Token::new(TokenType::IDENT, "result".to_string()),
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::IDENT, "add".to_string()),
            Token::new(TokenType::LPAREN, "(".to_string()),
            Token::new(TokenType::IDENT, "five".to_string()),
            Token::new(TokenType::COMMA, ",".to_string()),
            Token::new(TokenType::IDENT, "ten".to_string()),
            Token::new(TokenType::RPAREN, ")".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::EOF, "".to_string()),
        ];

        let mut lexer = Lexer::new(input);

        expected.iter().for_each(|t| {
            let token = lexer.next_token();
            assert_eq!(t, &token)
        });
    }

    #[test]
    fn test_next_token3() {
        let input = "\
let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;".to_string();

        let expected: Vec<Token> = vec![
            Token::new(TokenType::LET, "let".to_string()),
            Token::new(TokenType::IDENT, "five".to_string()),
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::INT, "5".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::LET, "let".to_string()),
            Token::new(TokenType::IDENT, "ten".to_string()),
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::INT, "10".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::LET, "let".to_string()),
            Token::new(TokenType::IDENT, "add".to_string()),
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::FUNCTION, "fn".to_string()),
            Token::new(TokenType::LPAREN, "(".to_string()),
            Token::new(TokenType::IDENT, "x".to_string()),
            Token::new(TokenType::COMMA, ",".to_string()),
            Token::new(TokenType::IDENT, "y".to_string()),
            Token::new(TokenType::RPAREN, ")".to_string()),
            Token::new(TokenType::LBRACE, "{".to_string()),
            Token::new(TokenType::IDENT, "x".to_string()),
            Token::new(TokenType::PLUS, "+".to_string()),
            Token::new(TokenType::IDENT, "y".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::RBRACE, "}".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::LET, "let".to_string()),
            Token::new(TokenType::IDENT, "result".to_string()),
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::IDENT, "add".to_string()),
            Token::new(TokenType::LPAREN, "(".to_string()),
            Token::new(TokenType::IDENT, "five".to_string()),
            Token::new(TokenType::COMMA, ",".to_string()),
            Token::new(TokenType::IDENT, "ten".to_string()),
            Token::new(TokenType::RPAREN, ")".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::BANG, "!".to_string()),
            Token::new(TokenType::MINUS, "-".to_string()),
            Token::new(TokenType::SLASH, "/".to_string()),
            Token::new(TokenType::ASTERISK, "*".to_string()),
            Token::new(TokenType::INT, "5".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::INT, "5".to_string()),
            Token::new(TokenType::LT, "<".to_string()),
            Token::new(TokenType::INT, "10".to_string()),
            Token::new(TokenType::GT, ">".to_string()),
            Token::new(TokenType::INT, "5".to_string()),
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
