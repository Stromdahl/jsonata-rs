use crate::Result;

#[derive(Debug, PartialEq)]
pub enum Token {
 Operator(String),
 String(String),
 Name(String),
}

pub type Source<'a> =  std::iter::Peekable<std::str::Chars<'a>>;

pub struct Lexer<'a> {
    pub source: Source<'a>,
    pub posotion: u32,
}

impl<'a> Lexer<'a> {

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

    fn next_token(&mut self) -> Option<Result<Token>> {
        let operators = [
            '$',
            '.'
        ];
        let token = if let Some(c) = self.source.next() {
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

                // names
                c => {
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
}
