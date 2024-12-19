pub use jsonata_error::{Result, Error};
use jsonata_expression::Expression;
use jsonata_parser::Parser;

mod evaluate;
use evaluate::evaluate;

mod environment;
use environment::{Binding, Environment};

mod data;
use data::JsonataData;

pub fn jsonata<T: JsonataData + Clone> (expr: &str) -> Result<Jsonata<T>> {
    let parser = Parser::new(expr);
    let ast = parser.parse()?;
    Ok(Jsonata::new(ast))
}


pub struct Jsonata<T> {
    ast: Expression,
    environment: Environment<T>,
}

impl<T: JsonataData + Clone> Jsonata<T> {

    pub fn new (ast: Expression) -> Self {
        let environment = Environment::new();
        Jsonata {
            ast,
            environment,
        }
    }

    pub fn bind(&mut self, name: String, binding: Binding<T>) {
        self.environment.bind(name, binding);
    }

    pub fn evaluate(&self, data: &T) -> Result<T> {
        evaluate(&self.ast, &data, &self.environment)
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
    use crate::environment::{Binding, Function};

    #[test]
    fn test_jsonata_function_bindings () -> Result<()> {
        let func = |_: Vec<serde_json::Value>| -> Result<serde_json::Value> {Ok(serde_json::json!(5.0))};

        // let mut expression = jsonata("$a() * x")?;  todo functions should expect "(args..)"
        let mut expression = jsonata("$a * x")?;
        expression.bind("a".into(), Binding::Function(Function {
            implementation: Box::new(func)
        }));
        let result = expression.evaluate(&serde_json::json!({"x": 4.0}))?;
        assert_eq!(result, serde_json::json!(20.0));
        Ok(())
    }


    #[test]
    fn test_jsonata_value_bindings () -> Result<()> {
        let mut expression = jsonata("$a * x")?;
        expression.bind("a".into(), Binding::Value(serde_json::json!(5.0)));
        let result = expression.evaluate(&serde_json::json!({"x": 4.0}))?;
        assert_eq!(result, serde_json::json!(20.0));
        Ok(())
    }

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
