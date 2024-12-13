use jsonata_error::{Error, Result};

mod lex;
use lex::{Lexer, Token};
pub use lex::Operator;

mod parse;
pub use parse::{Expression, Atom};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Parser {lexer: Lexer::new(source)}
    }

    pub fn parse(self) -> Result<Expression> {
        parse::parse(self.lexer)
    }
}
