pub enum NumericBinaryOperator {
    Add,
    Mult,
}

pub enum BinaryOperator {
    Numeric(NumericBinaryOperator)
}

pub enum Expr {
    Number(f64),
    Field(String),
    Binary(BinaryOperator, Box<Expr>, Box<Expr>),
    Chain(Box<Expr>, Box<Expr>),
}
