use crate::lexer::Lexer;
use crate::tokens::Token;
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

    fn parse_program(&self) {
        todo!()
    }
}
