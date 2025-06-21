use crate::tokens::{Token, TokenType};

enum Node {
    Program(ProgramNode),
    Statement(StatementNode),
    Expression(ExpressionNode),
}

struct ProgramNode {
    statements: Vec<StatementNode>
}

impl Node {
    fn token_literal(&self) -> String {
        match self {
            Node::Program(p) => p.statements.first().unwrap_or(return "".to_string()).token_literal(),
            Node::Statement(s) => todo!(),
            Node::Expression(e) => todo!(),
        }
    }
}

enum StatementNode {
    Let(LetStatement)
}

impl StatementNode {
    fn token_literal(&self) -> String {
        match self {
            Self::Let(s) => s.token.literal.clone(),
        }
    }
    fn statment_node(&self) { todo!() }
}

struct LetStatement {
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

struct IdentifierExpression {
    token: Token,
    value: String,
}

impl IdentifierExpression {
    fn token_literal(&self) -> String { todo!() }
    fn expression_node(&self) { todo!() }
}
