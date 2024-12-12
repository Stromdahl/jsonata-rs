mod error;
pub use error::{Error, Result};

mod lex;
use lex::{Lexer, Token};

mod parse;
use crate::parse::S;

mod evaluate;
use evaluate::evaluate;


pub struct Jsonata(S);

pub fn jsonata (expr: &str) -> Result<Jsonata> {
    let mut lexer = Lexer::new(expr);
    let ast = parse::parse(&mut lexer)?;
    Ok(Jsonata(ast))
}

impl Jsonata {
    pub fn evaluate(&self) -> Result<()> {
        println!("{}", evaluate(self.0.clone())?);
        Ok(())
    }
}
