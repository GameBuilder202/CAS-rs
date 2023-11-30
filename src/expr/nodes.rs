use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    // (Coeff, Term)
    pub terms: Vec<(f64, Term)>,
}

#[derive(Debug, Clone)]
pub struct Term {
    // (Base, Exponent)
    pub factors: Vec<(Factor, Expr)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Factor {
    Var(char),
    Num(f64),
    Func(Func),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Func {
    Sqrt(Expr),

    // Trig functions
    Sin(Expr),
    Cos(Expr),
    Tan(Expr),
}

pub trait IsConst {
    fn get_const(&self) -> Option<f64>;
}

impl IsConst for Expr {
    fn get_const(&self) -> Option<f64> {
        if self.terms.len() == 1 && self.terms[0].1.factors.is_empty() {
            let term = &self.terms[0];
            Some(term.0)
        } else {
            None
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.terms
                .iter()
                .map(|term| if term.1.factors.is_empty() {
                    term.0.to_string()
                } else {
                    format!("{} * {}", term.0, term.1)
                })
                .collect::<Vec<_>>()
                .join(" + ")
        )?;

        Ok(())
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.factors
                .iter()
                .map(|factor| format!("{}^{}", factor.0, factor.1))
                .collect::<Vec<_>>()
                .join(" * ")
        )?;

        Ok(())
    }
}

impl Display for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var(v) => write!(f, "{}", v)?,
            Self::Num(n) => write!(f, "{}", n)?,
            Self::Func(func) => write!(f, "{}", func)?,
            Self::Expr(expr) => write!(f, "{}", expr)?,
        }

        Ok(())
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sqrt(expr) => write!(f, "sqrt({})", expr)?,

            Self::Sin(expr) => write!(f, "sin({})", expr)?,
            Self::Cos(expr) => write!(f, "cos({})", expr)?,
            Self::Tan(expr) => write!(f, "tan({})", expr)?,
        }

        Ok(())
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        if self.factors.is_empty() {
            other.factors.is_empty()
        } else {
            self.factors.iter().all(|f| other.factors.contains(f))
        }
    }
}
