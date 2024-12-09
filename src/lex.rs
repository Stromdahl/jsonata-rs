use core::f64;

use crate::{Error, Result};

#[derive(Debug, PartialEq)]
pub enum Token {
 Operator(String), // todo use enum Operator?
 String(String), // todo use &str ?
 Name(String), // todo use enum Name?
 Number(f64), // This should be equal to javascript "Number" (IEEE 754-2019 binary64)
}

pub type Source<'a> =  std::iter::Peekable<std::str::Chars<'a>>;

pub struct Lexer<'a> {
    pub source: Source<'a>,
    pub posotion: u32,
}

impl<'a> Lexer<'a> {

    // todo impl std::str::FromStr
    pub fn from_str(source: &'a str) -> Self {
        let chars: Source<'a> = source.chars().peekable();
        Lexer::new(chars)
    }

    pub fn new (source: Source<'a>) -> Self {
        Self {
            source,
            posotion: 0,
        }
    }

    fn trim_while<F>(&mut self, f: F)
    where
        F: FnOnce(&char) -> bool + Copy,
    {
        while self.source.next_if(f).is_some() {}
    }

    fn next_token(&mut self) -> Option<Result<Token>> {
        let operators = [
            '$',
            '.',
            '+',
            '*',
        ];
        let token = if let Some(c) = self.source.next() {
            self.trim_while(|x| x.is_whitespace());
            match c {
                // single char operators
                c if operators.contains(&c)=> {
                    Ok(Token::Operator(c.to_string()))
                }

                // string literals
                '"' => {
                    let mut text = String::new();
                    while let Some(c) = self.source.next_if(|&c| c != '"') {
                        text.push(c);
                    };
                    Ok(Token::String(text))
                }

                // numeric literals
                // TODO: This is a placeholder implementation of numeric literal
                // It's problerbly not compatible with the javascript implementation
                // Needs tests to make sure
                '0'..='9' => {
                    let mut text = String::from(c);
                    while let Some(x) = self.source.next_if(|&x| x.is_numeric()) {
                        text.push(x)
                    }

                    if let Some(x) = self.source.next_if(|&x| x == '.') {
                        if let Some(&y) = self.source.peek() {
                            if y.is_numeric() {
                                text.push(x);
                            }

                            while let Some(x) = self.source.next_if(|&x| x.is_numeric()) {
                                text.push(x)
                            }
                        }
                    }
                    match text.parse() {
                        Ok(literal) => Ok(Token::Number(literal)),
                        Err(_) => Err(Error::S0102)
                    }
                }

                // names
                _ => {
                    let mut text = String::from(c);
                    while let Some(c) = self.source.next_if(|&c| !operators.contains(&c)) {
                        text.push(c)
                    };
                    match text.as_str() {
                        "or" | "in" | "and" => todo!(),
                        "true" => todo!(),
                        "false" => todo!(),
                        "null" => todo!(),
                        _ => {
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
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_simple_expression() -> Result<()> {
        let lexer = Lexer::from_str("$price.foo.bar");
        let tokens = lexer.collect::<Result<Vec<Token>>>()?;
        assert_eq!(tokens, [
            Token::Operator("$".into()),
            Token::Name("price".into()),
            Token::Operator(".".into()),
            Token::Name("foo".into()),
            Token::Operator(".".into()),
            Token::Name("bar".into())]
        );
        Ok(())
    }

    #[test]
    fn test_lex_numeric_expression() -> Result<()> {
        let lexer = Lexer::from_str("1 +2* 3");
        let tokens = lexer.collect::<Result<Vec<Token>>>()?;
        assert_eq!(tokens, [
            Token::Number(1.0),
            Token::Operator("+".into()),
            Token::Number(2.0),
            Token::Operator("*".into()),
            Token::Number(3.0),
        ]
        );
        Ok(())
    }
}
