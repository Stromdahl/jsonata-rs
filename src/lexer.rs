use crate::Result;

#[derive(Debug, PartialEq)]
pub enum Token {
 Operator(String),
 String(String),
 Name(String),
}

pub struct Lexer<Chars: Iterator<Item = char>> {
    pub source: std::iter::Peekable<Chars>,
    pub posotion: u32,
}

impl<Chars: Iterator<Item = char>> Lexer<Chars> {

    pub fn new (source: std::iter::Peekable<Chars>) -> Self {
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

impl<Chars: Iterator<Item = char>> Iterator for Lexer<Chars> {
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
        let expr = "$price.foo.bar";
        let lexer = Lexer::new(expr.chars().peekable());
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
