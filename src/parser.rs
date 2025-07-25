use crate::lexer::Lexer;
use crate::tokens::{Token, TokenType};
use crate::ast::{self, IdentifierExpression, InfixExpression};
use std::fmt::format;
use std::ops::Mul;
use std::str::MatchIndices;
use std::{mem, panic};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    None,
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

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
        let mut program = ast::ProgramNode::default();
        
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
                _ => {
                    if let Some(stmt) = self.parse_expression_statement(Precedence::Lowest) {
                        return Some(ast::StatementNode::Expression(stmt))
                    }
                }
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

    fn parse_expression_statement(&mut self, precedence: Precedence) -> Option<ast::ExpressionStatement> {
        let token = self.cur_token.clone().unwrap();
        let expression = match token.token_type {
            TokenType::Ident => {
                let ident = ast::IdentifierExpression::new(&token.literal);
                ast::ExpressionNode::Identifier(ident)
            },
            TokenType::Int => {
                let ident = ast::IntegerLiteralExpression::new(&token.literal.parse().unwrap());
                ast::ExpressionNode::IntegerLiteral(ident)
            }
            // parse prefix expression
            TokenType::Bang|TokenType::Minus => {
                let operator = token.literal.clone();
                self.next_token();
                let right = Rc::new(self.parse_expression_statement(Precedence::Prefix).unwrap().expression.unwrap());
                let ident = ast::PrefixExpression::new(token.clone(), operator, right);
                let mut left_expression = ast::ExpressionNode::Prefix(ident);

                while !self.next_token_is(TokenType::SemiColon) && (precedence < self.peek_precedence()) {
                    let peek_token = self.peek_token.clone().unwrap();
                    if !peek_token.token_type.is_infix_token() {
                        break;
                    }

                    self.next_token();

                    let cur_token = self.cur_token.clone().unwrap();
                    let operator = cur_token.literal.clone();
                    let left = Rc::new(left_expression.clone());
                    let prec = self.cur_precedence();
                    
                    self.next_token();

                    let right = Rc::new(self.parse_expression_statement(self.cur_precedence()).unwrap().expression.unwrap());

                    left_expression = ast::ExpressionNode::Infix(ast::InfixExpression::new(cur_token, operator, left, right))
                }

                left_expression
            },
            _ => {
                let msg = format!("no parsing logic found for token type \"{:?}\"", token.token_type);
                self.errors.push(msg);
                return None
            }
        };
        let statement = ast::ExpressionStatement::new(token, Some(expression));

        if self.next_token_is(TokenType::SemiColon) {
            self.next_token();
        }

        Some(statement)
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

    fn cur_precedence(&self) -> Precedence {
        match self.cur_token.clone().unwrap().token_type {
            TokenType::Eq | TokenType::NotEq => Precedence::Equals,
            TokenType::LT | TokenType::GT => Precedence::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Slash | TokenType::Asterisk => Precedence::Product,
            _ => Precedence::Lowest
        }
    }

    fn peek_precedence(&self) -> Precedence {
        match self.peek_token.clone().unwrap().token_type {
            TokenType::Eq | TokenType::NotEq => Precedence::Equals,
            TokenType::LT | TokenType::GT => Precedence::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Slash | TokenType::Asterisk => Precedence::Product,
            _ => Precedence::Lowest
        }
    }
}


#[cfg(test)]
mod tests {
    use std::panic;

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

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;".to_string();

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();

        check_parse_errors(p);

        assert_eq!(program.statements.len(), 1, "Program must contain 1 statement.");

        assert!(matches!(program.statements[0], ast::StatementNode::Expression(_)));
        match &program.statements[0] {
            ast::StatementNode::Expression(s) => {
                if let Some(e) = &s.expression {
                    match &e {
                        ast::ExpressionNode::Identifier(ie) => {
                            assert_eq!(ie.value, "foobar".to_string());
                            assert_eq!(ie.token_literal(), "foobar".to_string())
                        },
                        _ => {
                            panic!("Expression is not an identifier.")
                        }
                    }
                    
                } else {
                    panic!("Expression statement does not contain expression.")
                }
            },
            _ => panic!("program.statements[0] is not an expression statemnt.")
        };
        
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;".to_string();

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();

        check_parse_errors(p);

        assert_eq!(program.statements.len(), 1, "Program must contain 1 statement.");

        assert!(matches!(program.statements[0], ast::StatementNode::Expression(_)));
        match &program.statements[0] {
            ast::StatementNode::Expression(s) => {
                if let Some(e) = &s.expression {
                    match &e {
                        ast::ExpressionNode::IntegerLiteral(ie) => {
                            assert_eq!(ie.value, 5);
                            assert_eq!(ie.token_literal(), "5".to_string())
                        },
                        _ => {
                            panic!("Expression is not an identifier.")
                        }
                    }
                    
                } else {
                    panic!("Expression statement does not contain expression.")
                }
            },
            _ => panic!("program.statements[0] is not an expression statemnt.")
        };
        
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        struct PrefixTest {
            input: String,
            operator: String,
            integer_value: i64,
        };

        let prefix_tests = vec![
            PrefixTest { input: "!5;".to_string(), operator: "!".to_string(), integer_value: 5},
            PrefixTest { input: "-15;".to_string(), operator: "-".to_string(), integer_value: 15},
        ];

        prefix_tests.iter().for_each(|t| {
            let mut l = Lexer::new(t.input.to_string());
            let mut p = Parser::new(&mut l);

            let program = p.parse_program();

            check_parse_errors(p);

            assert_eq!(program.statements.len(), 1, "Program must contain 1 statement.");

            match &program.statements[0] {
                ast::StatementNode::Expression(s) => {
                    if let Some(e) = &s.expression {
                        match &e {
                            ast::ExpressionNode::Prefix(pe) => {
                                assert_eq!(t.operator, pe.operator);
                                // assert_eq!(t.integer_value, e.right);
                                assert_integer_literal(&pe.right, t.integer_value);
                            },
                            _ => {
                                panic!("Expression is prefix.")
                            }
                        }

                    } else {
                        panic!("Statement does not contain expression.")
                    }
                },
                _ => panic!("program.statements[0] is not an expression statemnt.")
            };
        });
    }
    
    #[test]
    fn test_parsing_infix_expressions() {
        struct PrefixTest {
            input: String,
            left_value: i64,
            operator: String,
            right_value: i64,
        };

        let prefix_tests = vec![
            PrefixTest {input: "5 + 5;".to_string(), left_value: 5, operator: "+".to_string(), right_value: 5},
            PrefixTest {input: "5 - 5;".to_string(), left_value: 5, operator: "-".to_string(), right_value: 5},
            PrefixTest {input: "5 * 5;".to_string(), left_value: 5, operator: "*".to_string(), right_value: 5},
            PrefixTest {input: "5 / 5;".to_string(), left_value: 5, operator: "/".to_string(), right_value: 5},
            PrefixTest {input: "5 > 5;".to_string(), left_value: 5, operator: ">".to_string(), right_value: 5},
            PrefixTest {input: "5 < 5;".to_string(), left_value: 5, operator: "<".to_string(), right_value: 5},
            PrefixTest {input: "5 == 5;".to_string(), left_value: 5, operator: "==".to_string() , right_value: 5},
            PrefixTest {input: "5 != 5;".to_string(), left_value: 5, operator: "!=".to_string(), right_value: 5},
        ];

        prefix_tests.iter().for_each(|t| {
            let mut l = Lexer::new(t.input.to_string());
            let mut p = Parser::new(&mut l);

            let program = p.parse_program();

            check_parse_errors(p);

            assert_eq!(program.statements.len(), 1, "Program must contain 1 statement.");

            match &program.statements[0] {
                ast::StatementNode::Expression(s) => {
                    if let Some(e) = &s.expression {
                        match &e {
                            ast::ExpressionNode::Infix(ie) => {
                                assert_eq!(t.operator, ie.operator);
                                assert_integer_literal(&ie.left, t.left_value);
                                assert_integer_literal(&ie.right, t.right_value);
                            },
                            _ => {
                                panic!("Expression is not an infix")
                            }
                        }

                    } else {
                        panic!("Statement does not contain an infix expression.")
                    }
                },
                _ => panic!("program.statements[0] is not an expression statemnt.")
            };
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

    fn assert_integer_literal(il: &Rc<ast::ExpressionNode>, value: i64) {
        match &**il {
            ast::ExpressionNode::IntegerLiteral(ile) => {
                assert_eq!(ile.value, value);
                assert_eq!(ile.token_literal(), value.to_string());
            },
            _ => panic!("Not an integer literal expression")
        }

    }

}
