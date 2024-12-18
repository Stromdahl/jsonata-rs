use jsonata_expression::{Atom, Expression, FunctionOperator, NumericBinaryOperator};
use jsonata_error::{Result, Error};

use crate::JsonataData;

fn evalute_numeric_binary<T: JsonataData>(op: &NumericBinaryOperator, lhs: &Expression, rhs: &Expression, data: &T) -> Result<T> {
    let lhs = evaluate(lhs, data)?.as_f64().ok_or(Error::T2001)?;
    let rhs = evaluate(rhs, data)?.as_f64().ok_or(Error::T2002)?;
    let res = match op {
        NumericBinaryOperator::Add => lhs + rhs,
        NumericBinaryOperator::Mul => lhs * rhs,
        NumericBinaryOperator::Sub => lhs - rhs,
        NumericBinaryOperator::Div => lhs / rhs,
        NumericBinaryOperator::Mod => lhs % rhs,
    };
    Ok(T::from_f64(res))
}

pub fn evaluate<T: JsonataData>(expr: &Expression, data: &T) -> Result<T> {
    match expr {
        Expression::Atom(Atom::Number(n)) => Ok(T::from_f64(*n)),
        Expression::Atom(Atom::Name(n)) => {
            match data.get_field(n) {
                Some(n) => Ok(n),
                None => todo!("What should happend here?, on name: {}", n)
            }
        },
        Expression::Atom(Atom::String(_s)) => todo!(),
        Expression::Atom(Atom::End) => todo!(),
        Expression::Path(lhs, rhs) => {
            let intermediate = evaluate(lhs, data)?;
            if intermediate.is_array() {
                let results: Vec<T> = intermediate
                    .as_array()
                    .unwrap() // todo Handle invalid array extraction
                    .iter()
                    .filter_map(|item| evaluate(rhs, item).ok()) // Apply rhs to each item in the array
                    .collect();

                Ok(T::from_array(results)) // Combine results back into an array
            } else {
                evaluate(rhs, &intermediate)
            }
        },
        Expression::BinaryNumeric(op, lhs, rhs) => evalute_numeric_binary(op, lhs, rhs, data),
        Expression::Unary(_op, _lhs) => {
            todo!();
        },
        Expression::Function(op, lhs) => {
            match op {
                FunctionOperator::Sum => {
                    let values = evaluate(lhs, data)?;
                    if !values.is_array() {
                        todo!("return error");
                    }

                    let sum: f64 = values
                        .as_array()
                        .unwrap() // todo!
                        .into_iter()
                        .filter_map(|v| v.as_f64())
                        .sum();
                    Ok(T::from_f64(sum))
                },
            }
        },
    }
}

