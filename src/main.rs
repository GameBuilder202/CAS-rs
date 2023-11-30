use cas_rs::expr::{eval::*, nodes::*};

fn main() {
    let expr = Expr {
        terms: vec![
            (
                1.0,
                Term {
                    factors: vec![
                        (
                            Factor::Var('x'),
                            Expr {
                                terms: vec![(
                                    1.0,
                                    Term {
                                        factors: Vec::new(),
                                    },
                                )],
                            },
                        ),
                        (
                            Factor::Var('y'),
                            Expr {
                                terms: vec![(
                                    1.0,
                                    Term {
                                        factors: Vec::new(),
                                    },
                                )],
                            },
                        ),
                    ],
                },
            ),
            (
                2.0,
                Term {
                    factors: vec![
                        (
                            Factor::Var('x'),
                            Expr {
                                terms: vec![(
                                    1.0,
                                    Term {
                                        factors: Vec::new(),
                                    },
                                )],
                            },
                        ),
                        (
                            Factor::Var('y'),
                            Expr {
                                terms: vec![(
                                    1.0,
                                    Term {
                                        factors: Vec::new(),
                                    },
                                )],
                            },
                        ),
                    ],
                },
            ),
            (
                1.0,
                Term {
                    factors: Vec::new(),
                },
            ),
        ],
    };

    println!("Original expr: {}", expr);

    let simpl = simplify(&expr);

    println!("Expr simplified: {}", simpl);
}
