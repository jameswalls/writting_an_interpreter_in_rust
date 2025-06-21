use crate::tokens::{Token, TokenType};

enum Node {
    Program(ProgramNode),
    Statement(StatementNode),
    Expression(ExpressionNode),
}

pub struct ProgramNode {
    pub statements: Vec<StatementNode>
}

impl Node {
    pub fn token_literal(&self) -> String {
        match self {
            Node::Program(p) => p.statements.first().unwrap_or(return "".to_string()).token_literal(),
            Node::Statement(s) => s.token_literal(),
            Node::Expression(e) => todo!(),
        }
    }
}

pub enum StatementNode {
    Let(LetStatement)
}

impl StatementNode {
    pub fn token_literal(&self) -> String {
        match self {
            Self::Let(s) => s.token.literal.clone(),
        }
    }
    fn statment_node(&self) { todo!() }
}

pub struct LetStatement {
    token: Token,
    name: IdentifierExpression,
    value: ExpressionNode,
}

enum ExpressionNode {
    Identifier(IdentifierExpression)
}
impl ExpressionNode {
    fn token_literal(&self) -> String { todo!() }
}

#[derive(Debug, PartialEq)]
pub struct IdentifierExpression {
    token: Token,
    value: String,
}

impl IdentifierExpression {
    pub fn new(identifier: &str) -> Self {
        let token = Token::new(TokenType::Ident, identifier.to_string());
        let value = identifier.to_string();
        IdentifierExpression { token, value }
    }

    fn token_literal(&self) -> String { todo!() }
    fn expression_node(&self) { todo!() }
}
