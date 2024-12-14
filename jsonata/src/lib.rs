use jsonata_error::{Result, Error};
use jsonata_expression::{Expression, NumericBinaryOperator, Atom};
use jsonata_parser::Parser;

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

fn evaluate<T: JsonataData>(expr: &Expression, data: &T) -> Result<T> {
    match expr {
        Expression::Atom(Atom::Number(n)) => Ok(T::from_f64(*n)),
        Expression::Atom(Atom::Name(n)) => {
            if let Some(value) = data.get_field(n) {
                Ok(value) 
            } else {
                todo!();
            }
        },
        Expression::Atom(Atom::String(_s)) => todo!(),
        Expression::Atom(Atom::End) => todo!(),
        Expression::Path(lhs, rhs) => {
            let intermediate = evaluate(lhs, data)?;
            evaluate(rhs, &intermediate)
        },
        Expression::BinaryNumeric(op, lhs, rhs) => evalute_numeric_binary(op, lhs, rhs, data),
        Expression::Unary(_op, _lhs) => {
            todo!();
        },
        Expression::Function(_, _) => todo!(),
    }
}

pub struct Jsonata(Expression);

pub fn jsonata (expr: &str) -> Result<Jsonata> {
    let parser = Parser::new(expr);
    let ast = parser.parse()?;
    Ok(Jsonata(ast))
}

impl Jsonata {
    pub fn evaluate<T: JsonataData>(&self, data: &T) -> Result<T> {
        evaluate(&self.0, &data)
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

#[cfg(test)]
mod tests {
    use jsonata_error::Result;
    use crate::jsonata;

    #[test]
    fn test_jsonata_simple_expression () -> Result<()> {
        let data = serde_json::json!({
            "y": {"b": 5},
            "x": {"a": 5},
        });

        let expression = jsonata("x.a * y.b")?;
        let result = expression.evaluate(&data)?;
        assert_eq!(result, serde_json::json!(25.0));
        Ok(())
    }

    #[ignore = "sum and array not implemented yet"]
    #[test]
    fn test_jsonata_example() -> Result<()> {

        let data = serde_json::json!({
             "example": [
                 {"value": 4},
                 {"value": 7},
                 {"value": 13}
             ]
        });
        
        let expression = jsonata("$sum(example.value)")?;
        let result = expression.evaluate(&data)?;
        assert_eq!(result, serde_json::json!(24.0));
        Ok(())
    }
}
