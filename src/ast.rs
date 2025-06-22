use crate::tokens::{Token, TokenType};

enum Node {
    Program(ProgramNode),
    Statement(StatementNode),
    Expression(ExpressionNode),
}

impl Node {
    pub fn token_literal(&self) -> String {
        match self {
            Self::Program(p) => p.statements.first().unwrap_or(return "".to_string()).token_literal(),
            Self::Statement(s) => s.token_literal(),
            Self::Expression(e) => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct ProgramNode {
    pub statements: Vec<StatementNode>
}

impl ProgramNode {
    pub fn new() -> Self {
        ProgramNode { statements: Vec::new() }
    }
}

#[derive(Debug)]
pub enum StatementNode {
    Let(LetStatement),
    Return(ReturnStatement),
}

impl StatementNode {
    pub fn token_literal(&self) -> String {
        match self {
            Self::Let(s) => s.token.literal.clone(),
            Self::Return(s) => s.token.literal.clone(),
        }
    }
    fn statment_node(&self) { todo!() }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: IdentifierExpression,
    pub value: Option<ExpressionNode>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Option<ExpressionNode>,
}

#[derive(Debug)]
pub enum ExpressionNode {
    Identifier(IdentifierExpression)
}
impl ExpressionNode {
    fn token_literal(&self) -> String { todo!() }
}

#[derive(Debug, PartialEq)]
pub struct IdentifierExpression {
    token: Token,
    pub value: String,
}

impl IdentifierExpression {
    pub fn new(identifier: &str) -> Self {
        let token = Token::new(TokenType::Ident, identifier.to_string());
        let value = identifier.to_string();
        IdentifierExpression { token, value }
    }

    pub fn token_literal(&self) -> String { self.value.to_string() }
    fn expression_node(&self) { todo!() }
}
