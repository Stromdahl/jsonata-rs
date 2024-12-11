use crate::lex::Operator;
use crate::parse::{Atom, S};
use crate::Error;
use crate::Result;

fn evaluate_binary(op: Operator, lhs: S, rhs: S) -> Result<S> {
    let lhs = evaluate(lhs)?;
    let rhs = evaluate(rhs)?;

    let result = match op {
        Operator::Plus | Operator::Star | Operator::Minus | Operator::Slash | Operator::Percentage=> {
            S::Atom(evaluate_numeric_expression(op, lhs, rhs)?)
        }
        Operator::Dollar => todo!(),
        Operator::Dot => todo!(),
        Operator::ParenRight => todo!(),
        Operator::ParenLeft => todo!(),
    };
    Ok(result)
}

fn evaluate_unary(op: Operator, lhs: S) -> Result<S> {
    let lhs = evaluate(lhs)?;

    let result = match op {
         Operator::Minus=> {
            match lhs {
                S::Atom(Atom::Number(n)) => Atom::Number(-n),
                _ => return Err(Error::D1002)
            }
        }
        _ => panic!("Unexpected operator {}", op)
    };
    Ok(S::Atom(result))
}

fn evaluate_numeric_expression(op: Operator, lhs: S, rhs: S) -> Result<Atom> {
    let lhs = match lhs {
        S::Atom(Atom::Number(n)) => n,
        _ => return Err(Error::T2001),
    };
    let rhs = match rhs {
        S::Atom(Atom::Number(n)) => n,
        _ => return Err(Error::T2002),
    };

    let result = match op {
        Operator::Plus => lhs + rhs,
        Operator::Star => lhs * rhs,
        Operator::Minus => lhs - rhs,
        Operator::Slash => lhs / rhs,
        Operator::Percentage => lhs % rhs,
        _ => unreachable!("by the evaluate_binary match arm pattern")
    };
    Ok(Atom::Number(result))
}

pub fn evaluate(r: S) -> Result<S> {
    let res = match r {
        S::Atom(a) => S::Atom(a),
        S::Binary(op, lhs, rhs) => evaluate_binary(op, *lhs, *rhs)?,
        S::Unary(op, lhs) => evaluate_unary(op, *lhs)?,
    };
    Ok(res)
}
