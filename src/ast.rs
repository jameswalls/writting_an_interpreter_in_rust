use crate::tokens::{Token, TokenType};

enum Node {
    Program(ProgramNode),
    Statement(StatementNode),
    Expression(ExpressionNode),
}

struct ProgramNode {
    statements: Vec<Statement>
}
struct StatementNode {}
struct ExpressionNode {}

impl Node {
    fn token_literal(self) -> String {
        match self {
            Node::Program(p) => p.statements.first().unwrap_or(return "".to_string()).token_literal(),
            Node::Statement(s) => todo!(),
            Node::Expression(e) => todo!(),
        }
    }
}

enum Statement {
    Let(LetStatement),
}

impl Statement {
   fn statement_node(&self) { todo!() }
   fn token_literal(&self) -> String { todo!() }
}

enum Expression {}


struct LetStatement {
    token: Token,
    name: String,
    value: Expression,
}
