pub enum BinaryOperator {
    Add,
}

pub enum Expr {
    Number(f64),
    Field(String),
    Binary(BinaryOperator, Box<Expr>, Box<Expr>),
    Chain(Box<Expr>, Box<Expr>),
}
