
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Plus,
    Slash,
    Star,
    Dot,
    ParenRight,
    ParenLeft,
    Minus,
    Percentage,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Star => write!(f, "*"),
            Operator::Percentage => write!(f, "%"),
            Operator::Slash => write!(f, "/"),
            Operator::Dot => write!(f, "."),
            Operator::ParenRight => write!(f, ")"),
            Operator::ParenLeft => write!(f, "("),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Function {
    Sum
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'a> {
 Operator(Operator), // todo use enum Operator?
 String(&'a str),
 Name(&'a str), // todo use enum Name?
 Number(f64), // This should be equal to javascript "Number" (IEEE 754-2019 binary64)
 Variable(&'a str), 
 Function(Function)
}

