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

    fn read_char(&mut self) {
        self.ch = *self.input.as_bytes().get(self.read_position).unwrap_or(&0);
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
       *self.input.as_bytes().get(self.read_position).unwrap_or(&0)
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        // TODO: refactor so this returns (TokenType, literal) and call Token::new once afterwards
        let token = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Some(Token::new(TokenType::Eq, "==".to_string()))
                }
                else {
                    Some(Token::new(TokenType::Assign, "=".to_string()))
                }
            },
            b';' => Some(Token::new(TokenType::SemiColon, ";".to_string())),
            b'(' => Some(Token::new(TokenType::LParen, "(".to_string())),
            b')' => Some(Token::new(TokenType::RParen, ")".to_string())),
            b',' => Some(Token::new(TokenType::Comma, ",".to_string())),
            b'+' => Some(Token::new(TokenType::Plus, "+".to_string())),
            b'-' => Some(Token::new(TokenType::Minus, "-".to_string())),
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Some(Token::new(TokenType::NotEq, "!=".to_string()))
                }
                else {
                    Some(Token::new(TokenType::Bang, "!".to_string()))
                }
            },
            b'*' => Some(Token::new(TokenType::Asterisk, "*".to_string())),
            b'/' => Some(Token::new(TokenType::Slash, "/".to_string())),
            b'<' => Some(Token::new(TokenType::LT, "<".to_string())),
            b'>' => Some(Token::new(TokenType::GT, ">".to_string())),
            b'{' => Some(Token::new(TokenType::LBrace, "{".to_string())),
            b'}' => Some(Token::new(TokenType::RBrace, "}".to_string())),
            0 => None,
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = Token::lookup_indent(&literal);
                    return Some(Token::new(token_type, literal))
                } else if is_digit(self.ch) {
                    let literal = self.read_number();
                    let token_type = TokenType::Int;
                    return Some(Token::new(token_type, literal))
                }
                else {
                    Some(Token::new(TokenType::Illegal, (self.ch as char).to_string()))
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
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Plus, "+".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::SemiColon, ";".to_string()),
        ];

        let mut lexer = Lexer::new(input);

        expected.iter().for_each(|t| {
            assert_eq!(Some(t), lexer.next_token().as_ref())
        });
        assert_eq!(None, lexer.next_token())
    }

    #[test]
    fn test_next_token_2() {
        let input = "\
let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
".to_string();

        let expected: Vec<Token> = vec![
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "five".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Int, "5".to_string()),
            Token::new(TokenType::SemiColon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "ten".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::SemiColon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "add".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Function, "fn".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::Ident, "x".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Ident, "y".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::Ident, "x".to_string()),
            Token::new(TokenType::Plus, "+".to_string()),
            Token::new(TokenType::Ident, "y".to_string()),
            Token::new(TokenType::SemiColon, ";".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::SemiColon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "result".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Ident, "add".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::Ident, "five".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Ident, "ten".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::SemiColon, ";".to_string()),
            Token::new(TokenType::Bang, "!".to_string()),
            Token::new(TokenType::Minus, "-".to_string()),
            Token::new(TokenType::Slash, "/".to_string()),
            Token::new(TokenType::Asterisk, "*".to_string()),
            Token::new(TokenType::Int, "5".to_string()),
            Token::new(TokenType::SemiColon, ";".to_string()),
            Token::new(TokenType::Int, "5".to_string()),
            Token::new(TokenType::LT, "<".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::GT, ">".to_string()),
            Token::new(TokenType::Int, "5".to_string()),
            Token::new(TokenType::SemiColon, ";".to_string()),
            Token::new(TokenType::If, "if".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::Int, "5".to_string()),
            Token::new(TokenType::LT, "<".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::Return, "return".to_string()),
            Token::new(TokenType::True, "true".to_string()),
            Token::new(TokenType::SemiColon, ";".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::Else, "else".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::Return, "return".to_string()),
            Token::new(TokenType::False, "false".to_string()),
            Token::new(TokenType::SemiColon, ";".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::Eq, "==".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::SemiColon, ";".to_string()),
            Token::new(TokenType::Int, "10".to_string()),
            Token::new(TokenType::NotEq, "!=".to_string()),
            Token::new(TokenType::Int, "9".to_string()),
            Token::new(TokenType::SemiColon, ";".to_string()),
        ];

        let mut lexer = Lexer::new(input);

        expected.iter().for_each(|t| {
            assert_eq!(Some(t), lexer.next_token().as_ref())
        });
        assert_eq!(None, lexer.next_token())
    
    }
}
