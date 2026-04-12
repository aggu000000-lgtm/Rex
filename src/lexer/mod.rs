mod token;

pub use token::*;

use logos::Logos;

#[derive(Logos, Debug, Clone)]
#[logos(skip r"[ \t\n\r]+")]
pub enum RexToken {
    #[token("make")]
    Make,
    #[token("show")]
    Show,
    #[token("when")]
    When,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("loop")]
    Loop,
    #[token("func")]
    Func,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("null")]
    Null,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r"[0-9]+(\.[0-9]+)?")]
    Number,

    #[regex(r#""[^"]*""#)]
    String,

    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("=")]
    Equals,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("^")]
    Caret,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("!")]
    Bang,
    #[token("&")]
    Ampersand,
    #[token("|")]
    Pipe,
    #[token("?")]
    Question,
    #[token(":")]
    Colon,
}

pub struct Lexer<'a> {
    logos: logos::Lexer<'a, RexToken>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            logos: RexToken::lexer(input),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.logos.next() {
            match token {
                Ok(RexToken::Make) => tokens.push(Token::Keyword(Keyword::Make)),
                Ok(RexToken::Show) => tokens.push(Token::Keyword(Keyword::Show)),
                Ok(RexToken::When) => tokens.push(Token::Keyword(Keyword::When)),
                Ok(RexToken::If) => tokens.push(Token::Keyword(Keyword::If)),
                Ok(RexToken::Else) => tokens.push(Token::Keyword(Keyword::If)),
                Ok(RexToken::Loop) => tokens.push(Token::Keyword(Keyword::Loop)),
                Ok(RexToken::Func) => tokens.push(Token::Keyword(Keyword::Func)),
                Ok(RexToken::Return) => tokens.push(Token::Keyword(Keyword::Return)),
                Ok(RexToken::True) => tokens.push(Token::Keyword(Keyword::True)),
                Ok(RexToken::False) => tokens.push(Token::Keyword(Keyword::False)),
                Ok(RexToken::Null) => tokens.push(Token::Keyword(Keyword::Null)),
                Ok(RexToken::Identifier) => {
                    tokens.push(Token::Identifier(self.logos.slice().to_string()))
                }
                Ok(RexToken::Number) => {
                    let num: f64 = self.logos.slice().parse().unwrap_or(0.0);
                    tokens.push(Token::Number(num))
                }
                Ok(RexToken::String) => {
                    let s = self.logos.slice();
                    let content = &s[1..s.len() - 1];
                    tokens.push(Token::String(content.to_string()))
                }
                Ok(RexToken::OpenParen) => tokens.push(Token::Symbol(Symbol::OpenParen)),
                Ok(RexToken::CloseParen) => tokens.push(Token::Symbol(Symbol::CloseParen)),
                Ok(RexToken::OpenBrace) => tokens.push(Token::Symbol(Symbol::OpenBrace)),
                Ok(RexToken::CloseBrace) => tokens.push(Token::Symbol(Symbol::CloseBrace)),
                Ok(RexToken::OpenBracket) => tokens.push(Token::Symbol(Symbol::OpenBracket)),
                Ok(RexToken::CloseBracket) => tokens.push(Token::Symbol(Symbol::CloseBracket)),
                Ok(RexToken::Dot) => tokens.push(Token::Symbol(Symbol::Dot)),
                Ok(RexToken::Comma) => tokens.push(Token::Symbol(Symbol::Comma)),
                Ok(RexToken::Semicolon) => tokens.push(Token::Symbol(Symbol::Semicolon)),
                Ok(RexToken::Equals) => tokens.push(Token::Symbol(Symbol::Equals)),
                Ok(RexToken::Plus) => tokens.push(Token::Symbol(Symbol::Plus)),
                Ok(RexToken::Minus) => tokens.push(Token::Symbol(Symbol::Minus)),
                Ok(RexToken::Star) => tokens.push(Token::Symbol(Symbol::Star)),
                Ok(RexToken::Slash) => tokens.push(Token::Symbol(Symbol::Slash)),
                Ok(RexToken::Percent) => tokens.push(Token::Symbol(Symbol::Percent)),
                Ok(RexToken::Caret) => tokens.push(Token::Symbol(Symbol::Caret)),
                Ok(RexToken::LessThan) => tokens.push(Token::Symbol(Symbol::LessThan)),
                Ok(RexToken::GreaterThan) => tokens.push(Token::Symbol(Symbol::GreaterThan)),
                Ok(RexToken::Bang) => tokens.push(Token::Symbol(Symbol::Bang)),
                Ok(RexToken::Ampersand) => tokens.push(Token::Symbol(Symbol::Ampersand)),
                Ok(RexToken::Pipe) => tokens.push(Token::Symbol(Symbol::Pipe)),
                Ok(RexToken::Question) => tokens.push(Token::Symbol(Symbol::Question)),
                Ok(RexToken::Colon) => tokens.push(Token::Symbol(Symbol::Colon)),
                Err(_) => {
                    println!("Lexer error at position {}", self.logos.span().start);
                }
            }
        }
        tokens.push(Token::Eof);
        tokens
    }
}
