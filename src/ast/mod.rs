use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AstNode {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Make(String, Expression),
    Show(Expression),
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    Loop {
        variable: Option<String>,
        count: Option<Expression>,
        body: Vec<Statement>,
    },
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    Return(Expression),
    Expr(Expression),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Identifier(String),
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    Unary {
        operator: Operator,
        operand: Box<Expression>,
    },
    Call {
        function: String,
        arguments: Vec<Expression>,
    },
    Property {
        object: Box<Expression>,
        property: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
    And,
    Or,
    Not,
}

pub struct Ast;

impl Ast {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, tokens: Vec<crate::lexer::Token>) -> Result<Program, String> {
        println!("Parsing {} tokens...", tokens.len());
        Ok(Program {
            statements: Vec::new(),
        })
    }
}

impl Default for Ast {
    fn default() -> Self {
        Self::new()
    }
}
