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
    Cons(char, Vec<S>),
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


fn infix_binding_power () -> (u8, u8) {
    todo!()
}

fn expr_bp(mut lexer: Lexer) -> Result<S> {
     let lhs = match lexer.next() {
        Some(token) => token?,
        None => return Ok(S::Atom(Atom::End)),
    };

    let lhs = match lhs {
        Token::Number(n) => S::Atom(Atom::Number(n)),
        t => todo!("{t:?}")
    };

    loop {
        let op = lexer.peek();
        // let op = match lexer.next().unwrap().unwrap() {
        //     Token::Operator(op) => op,
        //     t => panic!("bad token: {:?}", t),
        // };
        // let (l_bp, r_bp) = infix_binding_power(op);
        // todo!()
    }

    Ok(lhs)
}

fn expr(lexer: Lexer<'_>) -> Result<S> {
    expr_bp(lexer)
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
            expr(Lexer::from_str(source))?.to_string(), 
            "1"
        );
        Ok(())
    }
}
