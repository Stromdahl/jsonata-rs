use crate::Operator;
use crate::Lexer;
use crate::Result;
use crate::Token;

// https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Number(f64),
    String(String),
    Name(String),
    End,
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(i) => write!(f, "{i}"),
            Self::Name(n) => write!(f, "{n}"),
            Self::String(n) => write!(f, "\"{n}\""),
            Self::End => write!(f, ""),
        }
    }
}

// https://en.wikipedia.org/wiki/S-expression
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Atom(Atom),
    Binary(Operator, Box<Expression>, Box<Expression>),
    Unary(Operator, Box<Expression>),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Atom(i) => write!(f, "{i}"),
            Expression::Binary(op, lhs, rhs) => write!(f, "({} {} {})", op, lhs, rhs),
            Expression::Unary(op, lhs) => write!(f, "({} {})", op, lhs),
        }
    }
}

fn prefix_binding_power(op: &Operator) -> ((), u8) {
    match op {
        Operator::Minus => ((), 5),
        _ => panic!("bad op: {:?}", op),
    }
}

fn infix_binding_power(op: &Operator) -> Option<(u8, u8)> {
    let res = match op {
        Operator::Plus | Operator::Minus => (1, 2),
        Operator::Star | Operator::Slash => (3, 4),
        Operator::Dot => (6, 5),
        _ => return None,
    };
    Some(res)
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> Result<Expression> {
    let lhs = match lexer.next() {
        Some(token) => token?,
        None => return Ok(Expression::Atom(Atom::End)),
    };

    let mut lhs = match lhs {
        Token::Number(n) => Expression::Atom(Atom::Number(n)),
        Token::Name(n) => Expression::Atom(Atom::Name(n.to_string())),
        Token::String(n) => Expression::Atom(Atom::String(n.to_string())),
        Token::Operator(Operator::ParenLeft) => {
            let lhs = expr_bp(lexer, 0)?;
            assert_eq!(
                lexer.next(),
                Some(Ok(Token::Operator(Operator::ParenRight)))
            );
            lhs
        }
        Token::Operator(op) => {
            let ((), r_bp) = prefix_binding_power(&op);
            let rhs = expr_bp(lexer, r_bp)?;
            Expression::Unary(op, Box::new(rhs))
        }
    };

    loop {
        let op = match lexer.peek().cloned() {
            Some(Ok(Token::Operator(op))) => op,
            Some(Err(e)) => return Err(e),
            None => break,
            t => panic!("Unexpected token: {:?}", t),
        };

        if let Some((l_bp, r_bp)) = infix_binding_power(&op) {
            if l_bp < min_bp {
                break;
            }

            lexer.next();

            let rhs = expr_bp(lexer, r_bp)?;
            lhs = Expression::Binary(op, Box::new(lhs), Box::new(rhs));
            continue;
        }
        break;
    }

    Ok(lhs)
}

pub fn parse(mut lexer: Lexer) -> Result<Expression> {
    expr_bp(&mut lexer, 0)
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::Lexer;
    use crate::Result;

    #[test]
    fn test_parse_parenthesised_expression() -> Result<()> {
        let lexer = Lexer::new("(((0)))");
        let r = parse(lexer)?;
        assert_eq!(r.to_string(), "0");

        let lexer = Lexer::new("(1 + 2) * 3");
        let r = parse(lexer)?;
        assert_eq!(r.to_string(), "(* (+ 1 2) 3)");

        Ok(())
    }

    #[test]
    fn test_parse_prefix() -> Result<()> {
        let lexer = Lexer::new("--1 * 2");
        let r = parse(lexer)?;
        assert_eq!(r.to_string(), "(* (- (- 1)) 2)");

        let lexer = Lexer::new("--f . g");
        let r = parse(lexer)?;
        assert_eq!(r.to_string(), "(- (- (. f g)))");

        let lexer = Lexer::new("-(34 + 35)");
        let r = parse(lexer)?;
        assert_eq!(r.to_string(), "(- (+ 34 35))");
        Ok(())
    }

    #[test]
    fn test_parse_expression_complex() -> Result<()> {
        let lexer = Lexer::new("1 + 2 + f . g . h * 3 * 4");
        let r = parse(lexer)?;
        assert_eq!(r.to_string(), "(+ (+ 1 2) (* (* (. f (. g h)) 3) 4))");
        Ok(())
    }

    #[test]
    fn test_parse_path_expression() -> Result<()> {
        let lexer = Lexer::new("price.foo.bar");
        let r = parse(lexer)?;
        assert_eq!(r.to_string(), "(. price (. foo bar))");

        let lexer = Lexer::new("price.\"my name\".bar");
        let r = parse(lexer)?;
        assert_eq!(r.to_string(), "(. price (. \"my name\" bar))");
        Ok(())
    }

    #[test]
    fn test_parse_numeric_expression() -> Result<()> {
        let lexer = Lexer::new("1");
        let r = parse(lexer)?;
        assert_eq!(r.to_string(), "1");

        let lexer = Lexer::new("1 + 2 * 3");
        let r = parse(lexer)?;
        assert_eq!(r.to_string(), "(+ 1 (* 2 3))");

        let lexer = Lexer::new("a + b * c * d + e");
        let r = parse(lexer)?;
        assert_eq!(r.to_string(), "(+ (+ a (* (* b c) d)) e)");

        Ok(())
    }
}
