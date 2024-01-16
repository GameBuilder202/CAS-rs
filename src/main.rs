use cas_rs::expr::eval::simplify;
use cas_rs::parser::parse;

fn main() {
    let expr = parse("1x + 2xy + 3");
    println!("Original expr: {}", expr);

    let simpl = simplify(&expr);

    println!("Expr simplified: {}", simpl);
}
