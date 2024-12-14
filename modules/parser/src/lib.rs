use jsonata_error::Result;
use jsonata_expression::Expression;

mod lex;
use lex::Lexer;

mod token;
mod parse;

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
