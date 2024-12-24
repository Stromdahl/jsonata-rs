#[derive(Debug)]
pub enum Atom {
    Number(f64),
    Name(String),
    String(String),
    End,
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(i) => write!(f, "{i}"),
            Self::Name(n) => write!(f, "{n}"),
            Self::String(n) => write!(f, "\"{n}\""),
            Self::End => write!(f, ""),
        }
    }
}


#[derive(Debug)]
pub enum NumericBinaryOperator {
    Add,
    Mul,
    Sub,
    Div,
    Mod,
}

impl std::fmt::Display for NumericBinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericBinaryOperator::Add => write!(f, "+"),
            NumericBinaryOperator::Mul => write!(f, "*"),
            NumericBinaryOperator::Sub => write!(f, "-"),
            NumericBinaryOperator::Div => write!(f, "/"),
            NumericBinaryOperator::Mod => write!(f, "%"),
        }
    }
}


#[derive(Debug)]
pub enum NumericUnaryOperator {
    Negate,
}

impl std::fmt::Display for NumericUnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericUnaryOperator::Negate => write!(f, "-"),
        }
    }
}

#[derive(Debug)]
pub enum Variable {
    Value,
    Function(Vec<Expression>)
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::Value => write!(f, ""),
            Variable::Function(args) => {
                let _ = write!(f, "(");
                for arg in args {
                    let _ = write!(f, "{arg}");
                }
                write!(f, ")")
            },
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Atom(Atom),
    BinaryNumeric(NumericBinaryOperator, Box<Expression>, Box<Expression>),
    Path(Box<Expression>, Box<Expression>),
    Unary(NumericUnaryOperator, Box<Expression>),
    Variable(String, Variable),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Atom(i) => write!(f, "{i}"),
            Expression::BinaryNumeric(op, lhs, rhs) => write!(f, "({} {} {})", op, lhs, rhs),
            Expression::Path(lhs, rhs) => write!(f, "(. {} {})", lhs, rhs),
            Expression::Unary(op, lhs) => write!(f, "({} {})", op, lhs),
            Expression::Variable(name, var) => write!(f, "${}{}", name, var),
        }
    }
}


