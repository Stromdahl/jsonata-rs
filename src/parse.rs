use crate::Token;

// https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html



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
    use crate::Lexer;
    use crate::Result;

    #[test]
    fn test_parse_simple () -> Result<()> {
        let lexer = Lexer::from_str("1");
        todo!()
    }
}
