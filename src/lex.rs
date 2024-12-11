
use crate::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Plus,
    Slash,
    Star,
    Dollar,
    Dot,
    ParenRight,
    ParenLeft,
    Minus,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Star => write!(f, "*"),
            Operator::Slash => write!(f, "/"),
            Operator::Dollar => write!(f, "$"),
            Operator::Dot => write!(f, "."),
            Operator::ParenRight => write!(f, ")"),
            Operator::ParenLeft => write!(f, "("),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'a> {
 Operator(Operator), // todo use enum Operator?
 String(&'a str),
 Name(&'a str), // todo use enum Name?
 Number(f64), // This should be equal to javascript "Number" (IEEE 754-2019 binary64)
}

pub struct Lexer<'a> {
    source: &'a str,
    position: usize,
    peeked: Option<Result<Token<'a>>>,
}

impl<'a> Lexer<'a> {

    pub fn new (source: &'a str) -> Self {
        Self {
            source,
            position: 0,
            peeked: None,
        }
    }

    fn peek_char (&self) -> Option<char> {
        self.source[self.position..].chars().next()
    }

    fn advance (&mut self) -> Option<char> {
        if let Some(c) = self.peek_char() {
            self.position += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    fn advance_while<F>(&mut self, predicate: F)
        where 
        F: Fn(char) -> bool,
    {
        while let Some(ch) = self.peek_char() {
            if !predicate(ch) {
                break;
            }
            if self.advance().is_none() {
                break;
            }
        }
    }

    pub fn peek (&mut self) -> Option<&Result<Token<'a>>> {
        if self.peeked.is_some() {
            return self.peeked.as_ref();
        }
        self.peeked = self.next();
        self.peeked.as_ref()
    }

    fn next_token (&mut self) -> Option<Result<Token<'a>>> {
        if let Some(peeked) = self.peeked.take() {
            return Some(peeked)
        }

        self.advance_while(|c| c.is_whitespace());
        let token = if let Some(c) = self.advance() {
            match c {
                // single char operators
                '+' => Ok(Token::Operator(Operator::Plus)),
                '-' => Ok(Token::Operator(Operator::Minus)),
                '*' => Ok(Token::Operator(Operator::Star)),
                '$' => Ok(Token::Operator(Operator::Dollar)),
                '.' => Ok(Token::Operator(Operator::Dot)),
                ')' => Ok(Token::Operator(Operator::ParenRight)),
                '(' => Ok(Token::Operator(Operator::ParenLeft)),

                // string literals
                // '"' => {
                //     let mut text = String::new();
                //     while let Some(c) = self.source.next_if(|&c| c != '"') {
                //         text.push(c);
                //     };
                //     Ok(Token::String(text))
                // },

                // numeric literals
                // TODO: This is a placeholder implementation of numeric literal
                // It's problerbly not compatible with the javascript implementation
                // Needs tests to make sure
                '0'..='9' => {
                    let start = self.position - 1;
                    self.advance_while(|c| c.is_numeric());
                    let end = self.position;
                    let text = &self.source[start..end];
                    match text.parse() {
                        Ok(literal) => Ok(Token::Number(literal)),
                        Err(_) => Err(Error::S0102)
                    }
                }

                // names
                _ => {
                    let start = self.position - 1;
                    self.advance_while(|c| c.is_alphanumeric());
                    let end = self.position;
                    let text = &self.source[start..end];
                    match text {
                        "or" | "in" | "and" => todo!(),
                        "true" => todo!(),
                        "false" => todo!(),
                        "null" => todo!(),
                        text => {
                            Ok(Token::Name(text))
                        }
                    }
                }

            }
        } else {
            return None
        };
        Some(token)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}


#[cfg(test)]
mod tests {
    use super::{Lexer, Operator, Result, Token};

    #[test]
    fn test_lex_prefix_number() -> Result<()> {
        let lexer = Lexer::new("-1");
        let tokens = lexer.collect::<Result<Vec<Token>>>()?;
        assert_eq!(tokens, [
            Token::Operator(Operator::Minus),
            Token::Number(1.0)
        ]);
        Ok(())
    }

    #[test]
    fn test_lex_single_number() -> Result<()> {
        let lexer = Lexer::new("1");
        let tokens = lexer.collect::<Result<Vec<Token>>>()?;
        assert_eq!(tokens, [
            Token::Number(1.0)
        ]);
        Ok(())
    }


    #[test]
    fn test_lex_simple_expression() -> Result<()> {
        let lexer = Lexer::new("$price.foo.bar");
        let tokens = lexer.collect::<Result<Vec<Token>>>()?;
        assert_eq!(tokens, [
            Token::Operator(Operator::Dollar),
            Token::Name("price"),
            Token::Operator(Operator::Dot),
            Token::Name("foo"),
            Token::Operator(Operator::Dot),
            Token::Name("bar")
        ]);
        Ok(())
    }

    #[test]
    fn test_lex_handle_whitespace() -> Result<()> {
        let lexer = Lexer::new("  foo   bar  ");
        let tokens = lexer.collect::<Result<Vec<Token>>>()?;
        assert_eq!(tokens, [
            Token::Name("foo"),
            Token::Name("bar")
        ]);
        Ok(())
    }

    #[test]
    fn test_lex_numeric_expression() -> Result<()> {
        let lexer = Lexer::new("1 +2* 3 ");
        let tokens = lexer.collect::<Result<Vec<Token>>>()?;
        assert_eq!(tokens, [
            Token::Number(1.0),
            Token::Operator(Operator::Plus),
            Token::Number(2.0),
            Token::Operator(Operator::Star),
            Token::Number(3.0),
        ]);
        Ok(())
    }

    #[ignore = "Deciaml support not implementet yet"]
    #[test]
    fn test_lex_numeric_decimal() -> Result<()> {
        let lexer = Lexer::new("1.1 2.2 3");
        let tokens = lexer.collect::<Result<Vec<Token>>>()?;
        assert_eq!(tokens, [
            Token::Number(1.1),
            Token::Number(2.2),
            Token::Number(3.0),
        ]);
        Ok(())
    }

    #[ignore = "Exponential support not implementet yet"]
    #[test]
    fn test_lex_numeric_exponentail() -> Result<()> {
        // JS: Number.parseFloat(123000).toExponential(2) -> "23e+5"
        let lexer = Lexer::new("23e+5");
        let tokens = lexer.collect::<Result<Vec<Token>>>()?;
        assert_eq!(tokens, [
            Token::Number(123000.0),
        ]);
        Ok(())
    }


}
