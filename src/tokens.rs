use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    pub literal: String,
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
            (String::from("fn"), TokenType::Function),
            (String::from("let"), TokenType::Let),
            (String::from("true"), TokenType::True),
            (String::from("false"), TokenType::False),
            (String::from("if"), TokenType::If),
            (String::from("else"), TokenType::Else),
            (String::from("return"), TokenType::Return),
        ]);

        if let Some(token_type) = keywords.get(ident) {
            token_type.clone()
        } else {
            TokenType::Ident
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Illegal,
    EOF,
    Ident,
    Int,
    Assign,
    // operators
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,
    Comma,
    SemiColon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Eq,
    NotEq,
    // keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}
