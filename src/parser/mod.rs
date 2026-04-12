use crate::ast::{AstNode, Expression, Operator, Program, Statement};
use crate::lexer::{Keyword, Symbol, Token};

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, tokens: Vec<Token>) -> Result<Program, String> {
        let mut tokens = tokens.into_iter().peekable();
        let mut statements = Vec::new();

        while let Some(token) = tokens.next() {
            match token {
                Token::Keyword(Keyword::Make) => {
                    if let Some(Token::Identifier(name)) = tokens.next() {
                        if let Some(Token::Symbol(Symbol::Equals)) = tokens.next() {
                            let expr = self.parse_expression(&mut tokens)?;
                            statements.push(Statement::Make(name, expr));
                        }
                    }
                }
                Token::Keyword(Keyword::Show) => {
                    let expr = self.parse_expression(&mut tokens)?;
                    statements.push(Statement::Show(expr));
                }
                Token::Eof => break,
                _ => {}
            }
        }

        Ok(Program { statements })
    }

    fn parse_expression<T: Iterator<Item = Token>>(
        &self,
        tokens: &mut std::iter::Peekable<T>,
    ) -> Result<Expression, String> {
        if let Some(token) = tokens.next() {
            match token {
                Token::Number(n) => Ok(Expression::Number(n)),
                Token::String(s) => Ok(Expression::String(s)),
                Token::Identifier(name) => Ok(Expression::Identifier(name)),
                Token::Keyword(Keyword::True) => Ok(Expression::Boolean(true)),
                Token::Keyword(Keyword::False) => Ok(Expression::Boolean(false)),
                Token::Keyword(Keyword::Null) => Ok(Expression::Null),
                _ => Err("Unexpected token in expression".to_string()),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
