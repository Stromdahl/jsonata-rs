use crate::Token;
use crate::Result;

pub struct Parser<Tokens: Iterator<Item = Token>> {
    pub tokens: std::iter::Peekable<Tokens>,
}

pub struct Ast;

impl<Tokens: Iterator<Item = Token>> Parser<Tokens> {

    pub fn new (tokens: std::iter::Peekable<Tokens>) -> Self {
        Self {
            tokens,
        }
    }
}

// impl Parser<I: Iterator<Item = Token>> {
//     pub fn new (tokens: &dyn std::iter::Iterator<Item = crate::Token>) -> Self {
//         Self {
//             tokens
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple () -> Result<()> {
        todo!()
    }
}
