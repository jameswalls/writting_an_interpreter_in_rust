use crate::lexer::Lexer;
use crate::tokens::Token;
use crate::ast;
use std::mem;


struct Parser<'a> {
    lexer: &'a mut Lexer,
    cur_token: Box<Token>,
    peek_token: Box<Token>,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer) -> Self {
        let cur_token = Box::new(lexer.next_token().unwrap());
        let peek_token = Box::new(lexer.next_token().unwrap());
        Parser { lexer, cur_token, peek_token }
    }

    fn next_token(&mut self) {
        self.cur_token = mem::replace(
            &mut self.peek_token,
            Box::new(self.lexer.next_token().unwrap())
        )
    }

    fn parse_program(&self) -> ast::ProgramNode {
        todo!()
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
        let p = Parser::new(&mut l);
        let program = p.parse_program();

        assert_eq!(program.statements.len(), 3, "Program must contain 3 statements");

        let tests = vec![
            "x".to_string(),
            "y".to_string(),
            "z".to_string(),
        ];

        tests
            .iter()
            .zip(program.statements.iter())
            .for_each(|(ident, s)| {
             });
    }

    fn assert_let_statement(s: ast::StatementNode, name: String) {
        assert_eq!(s.token_literal(), "let".to_string(), "Expected let statment")
    }
}
