use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Token {
    Identifier(String),
    Number(f64),
    String(String),
    Keyword(Keyword),
    Symbol(Symbol),
    Eof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Keyword {
    Make,
    Show,
    When,
    If,
    Loop,
    Func,
    Return,
    True,
    False,
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Symbol {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Dot,
    Comma,
    Colon,
    Semicolon,
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LessThan,
    GreaterThan,
    Bang,
    Ampersand,
    Pipe,
    Question,
}
