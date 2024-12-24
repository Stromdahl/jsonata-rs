use jsonata_expression::{Atom, Expression, NumericBinaryOperator};
use jsonata_error::{Result, Error};

use crate::{environment::{Binding, Environment, Function}, JsonataData};

fn evalute_numeric_binary<T: JsonataData + Clone>(op: &NumericBinaryOperator, lhs: &Expression, rhs: &Expression, data: &T, environment: &Environment<T>) -> Result<T> {
    let lhs = evaluate(lhs, data, environment)?.as_f64().ok_or(Error::T2001)?;
    let rhs = evaluate(rhs, data, environment)?.as_f64().ok_or(Error::T2002)?;
    let res = match op {
        NumericBinaryOperator::Add => lhs + rhs,
        NumericBinaryOperator::Mul => lhs * rhs,
        NumericBinaryOperator::Sub => lhs - rhs,
        NumericBinaryOperator::Div => lhs / rhs,
        NumericBinaryOperator::Mod => lhs % rhs,
    };
    Ok(T::from_f64(res))
}

pub fn evaluate<T: JsonataData + Clone>(expr: &Expression, data: &T, environment: &Environment<T>) -> Result<T> {
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
            let intermediate = evaluate(lhs, data, environment)?;
            if intermediate.is_array() {
                let results: Vec<T> = intermediate
                    .as_array()
                    .unwrap() // todo Handle invalid array extraction
                    .iter()
                    .filter_map(|item| evaluate(rhs, item, environment).ok()) // Apply rhs to each item in the array
                    .collect();

                Ok(T::from_array(results)) // Combine results back into an array
            } else {
                evaluate(rhs, &intermediate, environment)
            }
        },
        Expression::BinaryNumeric(op, lhs, rhs) => evalute_numeric_binary(op, lhs, rhs, data, environment),
        Expression::Unary(_op, _lhs) => {
            todo!();
        },
        Expression::Variable(name, variable) => {
            match variable {
                jsonata_expression::Variable::Value => {
                    match environment.lookup(name) {
                        Some(Binding::Value(value)) => Ok(value.clone()),
                        Some(Binding::Function(_)) => todo!("??"),
                        None => todo!("** no match **"),
                    }
                },
                jsonata_expression::Variable::Function(args) => {
                    match environment.lookup(name) {
                        Some(Binding::Value(_value)) => todo!("Error: Attempted to invoke a non-function"),
                        Some(Binding::Function(Function{implementation})) => {
                            let args: Result<Vec<T>> = args
                                .iter()
                                .map(|item| evaluate(item, data, environment))
                                .collect();
                            implementation(args?)
                        },
                        None => todo!("** no match **"),
                    }
                },
            }
        },
    }
}

