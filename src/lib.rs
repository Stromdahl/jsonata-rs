mod error;
pub use error::{Error, Result};

pub mod lex;
pub use lex::{Lexer, Token};

pub mod parse;

mod evaluate;
pub use evaluate::evaluate;
