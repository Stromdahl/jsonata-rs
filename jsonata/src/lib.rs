pub use jsonata_error::{Result, Error};
use jsonata_expression::Expression;
use jsonata_parser::Parser;

mod evaluate;
use evaluate::evaluate;

mod data;
use data::JsonataData;

pub struct Jsonata {
    ast: Expression,
}

pub fn jsonata (expr: &str) -> Result<Jsonata> {
    let parser = Parser::new(expr);
    let ast = parser.parse()?;
    Ok(Jsonata{ast})
}

impl Jsonata {
    pub fn evaluate<T: JsonataData>(&self, data: &T) -> Result<T> {
        evaluate(&self.ast, &data)
    }
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
        serde_json::Value::Number(serde_json::Number::from_f64(value).unwrap())
    }


    fn is_array(&self) -> bool {
        serde_json::Value::is_array(&self)
    }

    fn as_array(&self) -> Option<Vec<Self>>
        where Self: Sized {
         self.as_array().cloned()
    }

    fn from_array(array: Vec<Self>) -> Self {
        serde_json::Value::Array(array)
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
