use jsonata_expression::{Expr, BinaryOperator, NumericBinaryOperator};
use jsonata_error::{Result, Error};

fn evaluteNumericBinary<T: JsonataData>(op: &NumericBinaryOperator, lhs: &Expr, rhs: &Expr, data: &T) -> Result<T> {
    let lhs = evaluate(lhs, data)?.as_f64().ok_or(Error::T2001)?;
    let rhs = evaluate(rhs, data)?.as_f64().ok_or(Error::T2002)?;
    let res = match op {
        NumericBinaryOperator::Add => lhs + rhs,
        NumericBinaryOperator::Mult => lhs * rhs,
    };
    Ok(T::from_f64(res))
}

fn evaluate<T: JsonataData>(expr: &Expr, data: &T) -> Result<T> {
    match expr {
        Expr::Chain(lhs, rhs) => {
            let intermediate = evaluate(lhs, data)?;
            evaluate(rhs, &intermediate)
        }
        Expr::Number(n) => Ok(T::from_f64(*n)),
        Expr::Field(field) => {
            if let Some(value) = data.get_field(field) {
                Ok(value) 
            } else {
                todo!();
            }
        },
        Expr::Binary(op, lhs, rhs) => {
            let res = match op {
                BinaryOperator::Numeric(op) => evaluteNumericBinary(op, lhs, rhs, data)?,
            };
            Ok(res)
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

    use super::{Expr, BinaryOperator, NumericBinaryOperator, Result};

    #[test]
    fn serde_test () -> Result<()> {
        let value = serde_json::json!({
            "x": {"a": 2},
            "y": {"b": 5},
        });

        let expr = Expr::Binary(
            BinaryOperator::Numeric(NumericBinaryOperator::Mult),
            Box::new(Expr::Chain(
                Box::new(Expr::Field("x".into())),
                Box::new(Expr::Field("a".into()))
            )),
            Box::new(Expr::Chain(
                Box::new(Expr::Field("y".into())),
                Box::new(Expr::Field("b".into()))
            )),
        );

        let result = evaluate(&expr, &value)?;
        println!("Result = {:?}", result);
        Ok(())
    }
}
