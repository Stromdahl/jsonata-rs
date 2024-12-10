use crate::lex::Operator;
use crate::Token;
use crate::Lexer;
use crate::Result;

// https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html



pub struct Ast;

enum Atom {
    Number(f64),
    End,
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(i) => write!(f, "{i}"),
            Self::End => write!(f, ""),
        }
    }
}

// https://en.wikipedia.org/wiki/S-expression
pub enum S {
    Atom(Atom),
    Cons(Operator, Vec<S>),
}

impl std::fmt::Display for S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            S::Atom(i) => write!(f, "{i}"),
            S::Cons(head, rest) => {
                write!(f, "({head}")?;
                for s in rest {
                    write!(f, "{s}")?
                };
                write!(f, ")")
            }
        }
    }
}


fn infix_binding_power(op: &Operator) -> (u8, u8) {
    match op {
        Operator::Plus => (1, 2),
        Operator::Star => (3, 4),
        _ => panic!("bad op: {:?}", op)
    }
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> Result<S> {
     let lhs = match lexer.next() {
        Some(token) => token?,
        None => return Ok(S::Atom(Atom::End)),
    };

    let mut lhs = match lhs {
        Token::Number(n) => S::Atom(Atom::Number(n)),
        t => todo!("{t:?}")
    };

    loop {
        let op = match lexer.peek() {
            Some(Ok(Token::Operator(op))) => *op,
            Some(Err(e)) => return Err(e.clone()),
            None => break,
            t => panic!("bad token: {:?}", t),
        };
        let (l_bp, r_bp) = infix_binding_power(&op);
        if l_bp < min_bp {
            break;
        }
        lexer.next();
        let rhs = expr_bp(lexer, r_bp)?;
        lhs = S::Cons(op, vec![lhs, rhs])
    }

    Ok(lhs)
}

fn expr(lexer: &mut Lexer<'_>) -> Result<S> {
    expr_bp(lexer, 0)
}

#[cfg(test)]
mod tests {
    use super::expr;
    use crate::Lexer;
    use crate::Result;

    #[test]
    fn test_parse_single_number () -> Result<()> {
        let source = "1";
        assert_eq!(
            expr(&mut Lexer::new(source))?.to_string(), 
            "1"
        );
        Ok(())
    }
}
