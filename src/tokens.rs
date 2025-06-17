use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }

    pub fn lookup_indent(ident: &str) -> TokenType {

        let keywords = HashMap::from([
            (String::from("fn"), TokenType::FUNCTION),
            (String::from("let"), TokenType::LET),
            (String::from("true"), TokenType::TRUE),
            (String::from("false"), TokenType::FALSE),
            (String::from("if"), TokenType::IF),
            (String::from("else"), TokenType::ELSE),
            (String::from("return"), TokenType::RETURN),
        ]);

        if let Some(token_type) = keywords.get(ident) {
            token_type.clone()
        } else {
            TokenType::IDENT
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    // operators
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}
