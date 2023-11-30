use super::nodes::*;

pub fn eval(f: fn(char) -> Option<f64>, expr: &Expr) -> Expr {
    let replaced = Expr {
        terms: expr
            .terms
            .iter()
            .map(|(coeff, term)| {
                (
                    *coeff,
                    Term {
                        factors: term
                            .factors
                            .iter()
                            .map(|(base, exp)| {
                                (
                                    match base {
                                        Factor::Var(c) => {
                                            if let Some(n) = f(*c) {
                                                Factor::Num(n)
                                            } else {
                                                Factor::Var(*c)
                                            }
                                        }
                                        other => other.clone(),
                                    },
                                    eval(f, exp),
                                )
                            })
                            .collect(),
                    },
                )
            })
            .collect(),
    };

    collapse_consts(&replaced)
}

pub fn simplify(expr: &Expr) -> Expr {
    // TODO: Add more simplifications
    collapse_consts(expr)
}

fn collapse_consts(expr: &Expr) -> Expr {
    let mut expr = expr.clone();
    let mut collapsed_terms = vec![(
        0.0,
        Term {
            factors: Vec::new(),
        },
    )];

    for term in &mut expr.terms {
        let mut coeff = term.0;
        let mut non_const_factors = Vec::new();

        for factor in &mut term.1.factors {
            let mut coeffn = None;

            if let Factor::Num(n) = factor.0 {
                coeffn = Some(n)
            } else if let Factor::Func(func) = &mut factor.0 {
                match func {
                    Func::Sqrt(expr) => {
                        *expr = collapse_consts(expr);
                        if let Some(n) = expr.get_const() {
                            coeffn = Some(n.sqrt())
                        }
                    }

                    Func::Sin(expr) => {
                        *expr = collapse_consts(expr);
                        if let Some(n) = expr.get_const() {
                            coeffn = Some(n.sin())
                        }
                    }
                    Func::Cos(expr) => {
                        *expr = collapse_consts(expr);
                        if let Some(n) = expr.get_const() {
                            coeffn = Some(n.cos())
                        }
                    }
                    Func::Tan(expr) => {
                        *expr = collapse_consts(expr);
                        if let Some(n) = expr.get_const() {
                            coeffn = Some(n.tan())
                        }
                    }
                }
            }
            factor.1 = collapse_consts(&factor.1);

            if let Some(n) = &factor.1.get_const() {
                if let Some(coeffn) = coeffn {
                    coeff *= coeffn.powf(*n)
                } else {
                    non_const_factors.push(factor.clone())
                }
            } else {
                non_const_factors.push(factor.clone())
            }
        }

        let new_term = Term {
            factors: non_const_factors.clone(),
        };

        if non_const_factors.is_empty() {
            collapsed_terms[0].0 += coeff;
        } else if collapsed_terms
            .iter()
            .map(|(_, t)| t)
            .collect::<Vec<_>>()
            .contains(&&new_term)
        {
            let index = collapsed_terms
                .iter()
                .map(|(_, t)| t)
                .position(|t| *t == new_term)
                .unwrap();

            collapsed_terms[index].0 += coeff;
        } else {
            collapsed_terms.push((coeff, new_term))
        }
    }

    expr.terms = collapsed_terms;

    expr
}
