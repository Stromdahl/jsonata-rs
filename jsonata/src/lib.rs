use jsonata_expression::{Expression, BinaryOperator, NumericBinaryOperator, Atom};
use jsonata_error::{Result, Error};

fn evaluteNumericBinary<T: JsonataData>(op: &NumericBinaryOperator, lhs: &Expression, rhs: &Expression, data: &T) -> Result<T> {
    let lhs = evaluate(lhs, data)?.as_f64().ok_or(Error::T2001)?;
    let rhs = evaluate(rhs, data)?.as_f64().ok_or(Error::T2002)?;
    let res = match op {
        NumericBinaryOperator::Add => lhs + rhs,
        NumericBinaryOperator::Mult => lhs * rhs,
    };
    Ok(T::from_f64(res))
}

fn evaluate<T: JsonataData>(expr: &Expression, data: &T) -> Result<T> {
    match expr {
        Expression::Atom(Atom::Number(n)) => Ok(T::from_f64(*n)),
        Expression::Field(field) => {
            if let Some(value) = data.get_field(field) {
                Ok(value) 
            } else {
                todo!();
            }
        },
        Expression::Binary(op, lhs, rhs) => {
            match op {
                BinaryOperator::Numeric(op) => evaluteNumericBinary(op, lhs, rhs, data),
                BinaryOperator::Chain => {
                    let intermediate = evaluate(lhs, data)?;
                    evaluate(rhs, &intermediate)
                }
            }
        }
        Expression::Unary(_op, _lhs) => {
            todo!();
        }
    }
}

pub trait JsonataData {
    fn get_field(&self, field: &str) -> Option<Self>
        where 
            Self: Sized;

    fn as_f64(&self) -> Option<f64>;

    fn from_f64(value: f64) -> Self
        where Self: Sized;
}

impl JsonataData for serde_json::Value {
    fn get_field(&self, field: &str) -> Option<Self>
        where 
            Self: Sized {
        self.get(field).cloned()
    }

    fn as_f64(&self) -> Option<f64> {
        self.as_f64()
    }

    fn from_f64(value: f64) -> Self where Self: Sized {
        serde_json::json!(value)
    }
}

// pub struct Jsonata(Expression);
// 
// pub fn jsonata (expr: &str) -> Result<Jsonata> {
//     let parser = Parser::new(expr);
//     let ast = parser.parse()?;
//     Ok(Jsonata(ast))
// }
// 
// impl Jsonata {
//     pub fn evaluate(&self, data: serde_json::Value) -> Result<Expression> {
//         todo!()
//         //evaluate(self.0.clone())
//     }
// }

#[cfg(test)]
mod tests {

    use crate::evaluate;

    use super::{Expression, BinaryOperator as Binary, NumericBinaryOperator as Numeric, Result, Atom};

    #[test]
    fn serde_test () -> Result<()> {
        // let value: serde_json::Value = serde_json::from_str(r#"{"x": 3, "y": 3}"#).unwrap();
        let value = serde_json::json!({
            "y": {"b": 5},
        });

        // 5 * y.b;
        let expr = Expression::Binary(
            Binary::Numeric(Numeric::Mult),
            Box::new(Expression::Atom(Atom::Number(5.0))),
            Box::new(Expression::Binary(
                Binary::Chain,
                Box::new(Expression::Field("y".into())),
                Box::new(Expression::Field("b".into()))
            )),
        );
        let result = evaluate(&expr, &value)?;
        assert_eq!(result, serde_json::json!(25.0));
        Ok(())
    }
}
