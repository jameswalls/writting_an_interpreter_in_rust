use crate::lexer::Lexer;
use crate::tokens::{Token, TokenType};
use crate::ast;
use std::fmt::format;
use std::mem;


#[derive(Debug)]
pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let errors = Vec::new();
        Parser { lexer, cur_token, peek_token, errors }
    }

    fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn next_token(&mut self) {
        self.cur_token = mem::replace(
            &mut self.peek_token,
            self.lexer.next_token()
        )
    }

    pub fn parse_program(&mut self) -> ast::ProgramNode {
        let mut program = ast::ProgramNode::new(None);
        
        while let Some(cur_token) = &self.cur_token {

            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<ast::StatementNode> {
        if let Some(token) = &self.cur_token {
            match token.token_type {
                TokenType::Let => {
                    if let Some(stmt) = self.parse_let_statement() {
                        return Some(ast::StatementNode::Let(stmt))
                    }
                },
                TokenType::Return => {
                    if let Some(stmt) = self.parse_return_statement() {
                        return Some(ast::StatementNode::Return(stmt))
                    }
                },
                _ => {}
            }
        }     
        None
    }

    fn parse_let_statement(&mut self) -> Option<ast::LetStatement> {

        // TODO: use Rc to see if we can avoid cloining all along
        let token = self.cur_token.clone().unwrap();
        if !self.expect_peek(TokenType::Ident) {
            return None
        }
        let name = ast::IdentifierExpression::new(&self.cur_token.clone().unwrap().literal);

        if !self.expect_peek(TokenType::Assign) {
            return None
        }

        // TODO: skip until semicolon
        while !self.cur_token_is(TokenType::SemiColon) {
            self.next_token();
        }

        Some(ast::LetStatement { token, name, value: None })
    }

    fn parse_return_statement(&mut self) -> Option<ast::ReturnStatement> {

        // TODO: use Rc to see if we can avoid cloining all along
        let token = self.cur_token.clone().unwrap();
        self.next_token();

        // TODO: skip until semicolon
        while !self.cur_token_is(TokenType::SemiColon) {
            self.next_token();
        }

        Some(ast::ReturnStatement { token, value: None })
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
            self.peek_error(token_type);
            false
        }
    }

    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            token_type,
            self.peek_token.clone().unwrap().token_type
        );
        self.errors.push(msg);
    }
}


#[cfg(test)]
mod tests {
    use crate::ast::ProgramNode;

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

        check_parse_errors(p);

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

    #[test]
    fn test_return_statements() {
        let input = "\
return 5;
return 10;
return 993322;".to_string();
        
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();

        check_parse_errors(p);

        assert_eq!(program.statements.len(), 3, "Program must contain 3 statements");

        program.statements.iter()
            .for_each(|s| {
                assert_statement(s, "".to_string());
             });
    }

    fn assert_statement(stmt: &ast::StatementNode, name: String) {
        match stmt {
            ast::StatementNode::Let(s) => {
                assert_eq!("let".to_string(), stmt.token_literal(), "Token literal is not 'let'.");
                assert_eq!(name, s.name.value);
                assert_eq!(name, s.name.token_literal());
            },
            ast::StatementNode::Return(s) => {
                assert_eq!("return".to_string(), stmt.token_literal(), "Token literal is not 'return'.");
            },
            ast::StatementNode::Expression(expression_statement) => todo!(),
        }
    }

    fn check_parse_errors(p: Parser) {
        let errors = p.errors();
        if errors.len() == 0 {
            return
        }

        println!("parser has {} errors", errors.len());
        errors.iter().for_each(|e| println!("\tparser error: {}", e));
        panic!()
    }
}
