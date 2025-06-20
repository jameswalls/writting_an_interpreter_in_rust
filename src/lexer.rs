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
        let (token_type, literal) = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    (TokenType::Eq, "==".to_string())
                }
                else {
                    (TokenType::Assign, "=".to_string())
                }
            },
            b';' => (TokenType::SemiColon, ";".to_string()),
            b'(' => (TokenType::LParen, "(".to_string()),
            b')' => (TokenType::RParen, ")".to_string()),
            b',' => (TokenType::Comma, ",".to_string()),
            b'+' => (TokenType::Plus, "+".to_string()),
            b'-' => (TokenType::Minus, "-".to_string()),
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    (TokenType::NotEq, "!=".to_string())
                }
                else {
                    (TokenType::Bang, "!".to_string())
                }
            },
            b'*' => (TokenType::Asterisk, "*".to_string()),
            b'/' => (TokenType::Slash, "/".to_string()),
            b'<' => (TokenType::LT, "<".to_string()),
            b'>' => (TokenType::GT, ">".to_string()),
            b'{' => (TokenType::LBrace, "{".to_string()),
            b'}' => (TokenType::RBrace, "}".to_string()),
            0 => return None,
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
                    (TokenType::Illegal, (self.ch as char).to_string())
                }
            }
        };
        self.read_char();
        Some(Token::new(token_type, literal))
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

        let expected: Vec<(TokenType, String)> = vec![
            (TokenType::Assign, "=".to_string()),
            (TokenType::Plus, "+".to_string()),
            (TokenType::LParen, "(".to_string()),
            (TokenType::RParen, ")".to_string()),
            (TokenType::LBrace, "{".to_string()),
            (TokenType::RBrace, "}".to_string()),
            (TokenType::Comma, ",".to_string()),
            (TokenType::SemiColon, ";".to_string()),
        ];

        let mut lexer = Lexer::new(input);

        expected.iter().for_each(|(tt, l)| {
            assert_eq!(Some(Token::new(*tt, l.clone())), lexer.next_token())
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

        let expected: Vec<(TokenType, String)> = vec![
            (TokenType::Let, "let".to_string()),
            (TokenType::Ident, "five".to_string()),
            (TokenType::Assign, "=".to_string()),
            (TokenType::Int, "5".to_string()),
            (TokenType::SemiColon, ";".to_string()),
            (TokenType::Let, "let".to_string()),
            (TokenType::Ident, "ten".to_string()),
            (TokenType::Assign, "=".to_string()),
            (TokenType::Int, "10".to_string()),
            (TokenType::SemiColon, ";".to_string()),
            (TokenType::Let, "let".to_string()),
            (TokenType::Ident, "add".to_string()),
            (TokenType::Assign, "=".to_string()),
            (TokenType::Function, "fn".to_string()),
            (TokenType::LParen, "(".to_string()),
            (TokenType::Ident, "x".to_string()),
            (TokenType::Comma, ",".to_string()),
            (TokenType::Ident, "y".to_string()),
            (TokenType::RParen, ")".to_string()),
            (TokenType::LBrace, "{".to_string()),
            (TokenType::Ident, "x".to_string()),
            (TokenType::Plus, "+".to_string()),
            (TokenType::Ident, "y".to_string()),
            (TokenType::SemiColon, ";".to_string()),
            (TokenType::RBrace, "}".to_string()),
            (TokenType::SemiColon, ";".to_string()),
            (TokenType::Let, "let".to_string()),
            (TokenType::Ident, "result".to_string()),
            (TokenType::Assign, "=".to_string()),
            (TokenType::Ident, "add".to_string()),
            (TokenType::LParen, "(".to_string()),
            (TokenType::Ident, "five".to_string()),
            (TokenType::Comma, ",".to_string()),
            (TokenType::Ident, "ten".to_string()),
            (TokenType::RParen, ")".to_string()),
            (TokenType::SemiColon, ";".to_string()),
            (TokenType::Bang, "!".to_string()),
            (TokenType::Minus, "-".to_string()),
            (TokenType::Slash, "/".to_string()),
            (TokenType::Asterisk, "*".to_string()),
            (TokenType::Int, "5".to_string()),
            (TokenType::SemiColon, ";".to_string()),
            (TokenType::Int, "5".to_string()),
            (TokenType::LT, "<".to_string()),
            (TokenType::Int, "10".to_string()),
            (TokenType::GT, ">".to_string()),
            (TokenType::Int, "5".to_string()),
            (TokenType::SemiColon, ";".to_string()),
            (TokenType::If, "if".to_string()),
            (TokenType::LParen, "(".to_string()),
            (TokenType::Int, "5".to_string()),
            (TokenType::LT, "<".to_string()),
            (TokenType::Int, "10".to_string()),
            (TokenType::RParen, ")".to_string()),
            (TokenType::LBrace, "{".to_string()),
            (TokenType::Return, "return".to_string()),
            (TokenType::True, "true".to_string()),
            (TokenType::SemiColon, ";".to_string()),
            (TokenType::RBrace, "}".to_string()),
            (TokenType::Else, "else".to_string()),
            (TokenType::LBrace, "{".to_string()),
            (TokenType::Return, "return".to_string()),
            (TokenType::False, "false".to_string()),
            (TokenType::SemiColon, ";".to_string()),
            (TokenType::RBrace, "}".to_string()),
            (TokenType::Int, "10".to_string()),
            (TokenType::Eq, "==".to_string()),
            (TokenType::Int, "10".to_string()),
            (TokenType::SemiColon, ";".to_string()),
            (TokenType::Int, "10".to_string()),
            (TokenType::NotEq, "!=".to_string()),
            (TokenType::Int, "9".to_string()),
            (TokenType::SemiColon, ";".to_string()),
        ];

        let mut lexer = Lexer::new(input);

        expected.iter().for_each(|(tt, l)| {
            assert_eq!(Some(Token::new(*tt, l.clone())), lexer.next_token())
        });
        assert_eq!(None, lexer.next_token())
    
    }
}
