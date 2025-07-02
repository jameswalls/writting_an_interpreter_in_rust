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

    pub fn string(&self) -> String {
        match self {
            Node::Program(p) => {
                let buffer = String::new();
                p.statements.iter().map(|s| s.string()).collect()
            },
            Node::Statement(s) => s.string(),
            Node::Expression(e) => e.string(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ProgramNode {
    pub statements: Vec<StatementNode>
}

impl ProgramNode {
    pub fn new(statements: Vec<StatementNode>) -> Self {
        ProgramNode { statements }
    }
}

#[derive(Debug)]
pub enum StatementNode {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl StatementNode {
    pub fn token_literal(&self) -> String {
        match self {
            Self::Let(s) => s.token_literal(),
            Self::Return(s) => s.token_literal(),
            Self::Expression(s) => s.token_literal(),
        }
    }

    fn string(&self) -> String {
        let mut buffer = String::new();
        match self {
            StatementNode::Let(s) => {
                buffer.push_str(&s.token_literal());
                buffer.push_str(" ");
                buffer.push_str(&s.name.string());
                buffer.push_str(" = ");
                if let Some(v) = &s.value {
                    buffer.push_str(&v.string());
                }
                buffer.push_str(";");
            },
            StatementNode::Return(s) => {
                buffer.push_str(&s.token_literal());
                buffer.push_str(" ");
                if let Some(v) = &s.value {
                    buffer.push_str(&v.string());
                }
                buffer.push_str(";");
            },
            StatementNode::Expression(s) => {
                if let Some(v) = &s.expression {
                    buffer.push_str(&v.string());
                }
                buffer.push_str("");
            },
        };
        buffer
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: IdentifierExpression,
    pub value: Option<ExpressionNode>,
}

impl LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Option<ExpressionNode>,
}

impl ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<ExpressionNode>,
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Option<ExpressionNode>) -> Self {
        ExpressionStatement { token, expression }
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug)]
pub enum ExpressionNode {
    Identifier(IdentifierExpression),
    IntegerLiteral(IntegerLiteralExpression),
}
impl ExpressionNode {
    fn token_literal(&self) -> String { todo!() }
    fn string(&self) -> String { 
        match self {
            ExpressionNode::Identifier(i) => i.string(),
            ExpressionNode::IntegerLiteral(i) => i.string(),
        }
    }
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

    fn string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug)]
pub struct IntegerLiteralExpression {
    token: Token,
    pub value: i64,
}

impl IntegerLiteralExpression {
    pub fn new(identifier: &i64) -> Self {
        let token = Token::new(TokenType::Ident, identifier.to_string());
        IntegerLiteralExpression { token, value: *identifier }
    }

    pub fn token_literal(&self) -> String { self.value.to_string() }

    fn string(&self) -> String {
        self.value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let statement = StatementNode::Let(LetStatement { 
            token: Token::new(TokenType::Let, "let".to_string()),
            name: IdentifierExpression {
                token: Token::new(TokenType::Ident, "myVar".to_string()),
                value: "myVar".to_string() },
                value: Some(ExpressionNode::Identifier(IdentifierExpression {
                    token: Token::new(TokenType::Ident, "anotherVar".to_string()),
                    value: "anotherVar".to_string() 
                }))
        }) ;
        let mut program = Node::Program(ProgramNode::new(vec![statement]));
        
        assert_eq!(program.string(), "let myVar = anotherVar;".to_string())
    }
}
