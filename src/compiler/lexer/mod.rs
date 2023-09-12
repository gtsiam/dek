use logos::Logos;
use malachite::{
    num::conversion::{string::options::FromSciStringOptions, traits::FromSciString},
    Rational,
};
use miette::Diagnostic;
use thiserror::Error;

use super::source::{Source, Spanned};

#[derive(Debug, Logos)]
#[logos(error = TokenError)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token<'s> {
    #[token("=")]
    Assign,

    #[token("==")]
    Equals,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token(".")]
    Dot,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("let")]
    Let,

    #[token("in")]
    In,

    #[regex( // Binary
        r#"0b[01][01_]*"#,
        |lex| parse_number(&lex.slice()[2..], 2),
        priority = 4,
        ignore(ascii_case)
    )]
    #[regex( // Octal
        r#"0o[0-7][0-7_]*"#,
        |lex| parse_number(&lex.slice()[2..], 8),
        priority = 3,
        ignore(ascii_case)
    )]
    #[regex( // Hexadecimal
        r#"0x[0-9a-f][0-9a-f_]*"#,
        |lex| parse_number(&lex.slice()[2..], 16),
        priority = 2,
        ignore(ascii_case)
    )]
    #[regex( // Decimal (with scientific notation)
        r#"[+-]?[0-9][0-9]*(\.[0-9_]+)?([eE][+-]?[0-9_]+)?"#,
        |lex| parse_number(lex.slice(), 10),
        priority = 1,
        ignore(ascii_case)
    )]
    Number(Rational),

    #[regex(r#"[a-z_][a-z0-9_]*"#, ignore(ascii_case))]
    Identifier(&'s str),
}

#[derive(Debug, Clone, Error, Diagnostic, Default)]
pub enum TokenError {
    #[default]
    #[error("Unexpected token")]
    UnexpectedToken,

    #[error("Invalid number literal")]
    InvalidNumber,
}

impl PartialEq for TokenError {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for TokenError {}

pub struct Lexer {
    _priv: (),
}

impl Lexer {
    pub fn new() -> Self {
        Self { _priv: () }
    }

    pub fn lex<'s>(
        &self,
        source: &'s Source,
    ) -> impl Iterator<Item = Spanned<Result<Token<'s>, TokenError>>> + 's {
        Token::lexer(source.contents())
            .spanned()
            .map(|(token, range)| source.spanned(token, range))
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_number(number: &str, base: u8) -> Result<Rational, TokenError> {
    let number = number.replace('_', "");

    let mut options = FromSciStringOptions::default();
    options.set_base(base);

    Rational::from_sci_string_with_options(&number, options).ok_or(TokenError::InvalidNumber)
}
