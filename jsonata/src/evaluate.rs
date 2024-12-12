use jsonata_parser::Parser;
use jsonata_parser::Operator;
use jsonata_parser::Expression;
use jsonata_parser::Atom;
use jsonata_parser::Error;
use jsonata_parser::Result;

fn evaluate_binary(op: Operator, lhs: Expression, rhs: Expression) -> Result<Expression> {
    let lhs = evaluate(lhs)?;
    let rhs = evaluate(rhs)?;

    let is_numeric_operator = matches!(op, 
        Operator::Plus | 
        Operator::Star | 
        Operator::Minus |
        Operator::Slash |
        Operator::Percentage);
    let result = if is_numeric_operator {
        Expression::Atom(evaluate_numeric_expression(op, lhs, rhs)?)
    } else {
        todo!()
    };
    Ok(result)
}

fn evaluate_unary(op: Operator, lhs: Expression) -> Result<Expression> {
    let lhs = evaluate(lhs)?;

    let result = match op {
         Operator::Minus=> {
            match lhs {
                Expression::Atom(Atom::Number(n)) => Atom::Number(-n),
                _ => return Err(Error::D1002)
            }
        }
        _ => panic!("Unexpected operator {}", op)
    };
    Ok(Expression::Atom(result))
}

fn evaluate_numeric_expression(op: Operator, lhs: Expression, rhs: Expression) -> Result<Atom> {
    let lhs = match lhs {
        Expression::Atom(Atom::Number(n)) => n,
        _ => return Err(Error::T2001),
    };
    let rhs = match rhs {
        Expression::Atom(Atom::Number(n)) => n,
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

pub fn evaluate(r: Expression) -> Result<Expression> {
    let res = match r {
        Expression::Atom(a) => Expression::Atom(a),
        Expression::Binary(op, lhs, rhs) => evaluate_binary(op, *lhs, *rhs)?,
        Expression::Unary(op, lhs) => evaluate_unary(op, *lhs)?,
    };
    Ok(res)
}

