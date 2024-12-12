mod evaluate;
use evaluate::evaluate;

use jsonata_parser::Parser;
use jsonata_parser::Expression;
use jsonata_parser::Result;

pub struct Jsonata(Expression);

pub fn jsonata (expr: &str) -> Result<Jsonata> {
    let parser = Parser::new(expr);
    let ast = parser.parse()?;
    Ok(Jsonata(ast))
}

impl Jsonata {
    pub fn evaluate(&self) -> Result<Expression> {
        evaluate(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use jsonata_parser::Expression;

    use super::{jsonata, Result};

    #[test]
    fn jsonata_test() -> Result<()> {
        let res = jsonata("(1 + 2) * 3")?.evaluate()?;
        assert_eq!(res, Expression::Atom(jsonata_parser::Atom::Number(9.0)));
        Ok(())
    }
}
