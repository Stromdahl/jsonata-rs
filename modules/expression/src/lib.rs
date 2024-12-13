pub enum NumericBinaryOperator {
    Add,
    Mult,
}

pub enum BinaryOperator {
    Numeric(NumericBinaryOperator),
    Chain,
}

pub enum Atom {
    Number(f64),
}

pub enum Expression {
    Atom(Atom),
    Field(String),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Unary(BinaryOperator, Box<Expression>),
}
