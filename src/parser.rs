use crate::lexer::Lexer;
use crate::tokens::{Token, TokenType};
use crate::ast;
use std::mem;


#[derive(Debug)]
struct Parser<'a> {
    lexer: &'a mut Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser { lexer, cur_token, peek_token }
    }

    fn next_token(&mut self) {
        self.cur_token = mem::replace(
            &mut self.peek_token,
            self.lexer.next_token()
        )
    }

    fn parse_program(&mut self) -> ast::ProgramNode {
        let mut program = ast::ProgramNode::new();
        
        println!("parser {:?}", self);
        println!("program {:?}", program);
        while let Some(cur_token) = &self.cur_token {

            println!("cur token {:?}", cur_token);
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
                println!("updated program {:?}", program);
            } else {
                break;
            }
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<ast::StatementNode> {
        if let Some(token) = &self.cur_token {
            match token.token_type {
                TokenType::Illegal => todo!(),
                TokenType::EOF => todo!(),
                TokenType::Ident => todo!(),
                TokenType::Int => todo!(),
                TokenType::Assign => todo!(),
                TokenType::Plus => todo!(),
                TokenType::Minus => todo!(),
                TokenType::Bang => todo!(),
                TokenType::Asterisk => todo!(),
                TokenType::Slash => todo!(),
                TokenType::LT => todo!(),
                TokenType::GT => todo!(),
                TokenType::Comma => todo!(),
                TokenType::SemiColon => todo!(),
                TokenType::LParen => todo!(),
                TokenType::RParen => todo!(),
                TokenType::LBrace => todo!(),
                TokenType::RBrace => todo!(),
                TokenType::Eq => todo!(),
                TokenType::NotEq => todo!(),
                TokenType::Function => todo!(),
                TokenType::Let => {
                    println!("parsig statement for token {:?}", token);
                    if let Some(stmt) = self.parse_let_statement() {
                        println!("got stmt {:?}", stmt);
                        return Some(ast::StatementNode::Let(stmt))
                    }                 },
                TokenType::True => todo!(),
                TokenType::False => todo!(),
                TokenType::If => todo!(),
                TokenType::Else => todo!(),
                TokenType::Return => todo!(),
            }
        }     
        None
    }

    fn parse_let_statement(&mut self) -> Option<ast::LetStatement> {

        // TODO: use Rc to see if we can avoid cloining all along
        println!("parsing let statement {:?}", self);
        let token = self.cur_token.clone().unwrap();
        println!("\t token = {:?}", token);
        if !self.expect_peek(TokenType::Ident) {
            return None
        }
        let name = ast::IdentifierExpression::new(&self.cur_token.clone().unwrap().literal);
        println!("\t name = {:?}", name);

        if !self.expect_peek(TokenType::Assign) {
            return None
        }

        // TODO: skip until semicolon
        while !self.expect_peek(TokenType::SemiColon) {
            self.next_token();
        }

        println!("reached ;");
        Some(ast::LetStatement { token, name, value: None })
    }

    fn cur_token_is(&self, token_type: TokenType) -> bool {
        if let Some(t) = &self.cur_token {
            t.token_type == token_type
        } else {
            false
        }
    }

    fn next_token_is(&self, token_type: TokenType) -> bool {
        if let Some(t) = &self.peek_token {
            t.token_type == token_type
        } else {
            false
        }
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.next_token_is(token_type) {
            self.next_token();
            true
        } else {
            false
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_let_statements() {
        let input = "\
let x = 5;
let y = 10;
let foobar = 838383;".to_string();
        
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();

        assert_eq!(program.statements.len(), 3, "Program must contain 3 statements");

        let tests = vec![
            "x",
            "y",
            "foobar",
        ];

        tests
            .iter()
            .zip(program.statements.iter())
            .for_each(|(ident, s)| {
                assert_statement(s, ident.to_string());
             });
    }

    fn assert_statement(statement_node: &ast::StatementNode, name: String) {
        match statement_node {
            ast::StatementNode::Let(statement) => {
                assert_eq!("let".to_string(), statement_node.token_literal(), "Token literal is not 'let'.");
                assert_eq!(name, statement.name.value);
                assert_eq!(name, statement.name.token_literal());
            },
        }
    }
}
