use jsonata_expression::{Expr, BinaryOperator};
use jsonata_error::{Result, Error};

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
            let lhs = match evaluate(lhs, data)?.as_f64() {
                Some(n) => n,
                None => return Err(Error::T2001),
            };
            let rhs = match evaluate(rhs, data)?.as_f64() {
                Some(n) => n,
                None => return Err(Error::T2002),
            };
            let res = match op {
                BinaryOperator::Add => lhs + rhs,
            };
            Ok(T::from_f64(res))
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

    use super::{Expr, BinaryOperator, Result};

    #[test]
    fn serde_test () -> Result<()> {
        let value = serde_json::json!({
            "x": {"a": 2},
            "y": 2,
        });

        let expr = Expr::Binary(
            BinaryOperator::Add,
            Box::new(Expr::Chain(
                Box::new(Expr::Field("x".into())),
                Box::new(Expr::Field("a".into()))
            )),
            Box::new(Expr::Field("y".into())),
        );

        let result = evaluate(&expr, &value)?;
        println!("Result = {:?}", result);
        Ok(())
    }
}
