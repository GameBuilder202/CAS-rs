use cas_rs::algebra::{group::*, op::*, sets::*, *};
use cas_rs::expr::eval::simplify;
use cas_rs::parser::parse;

fn main() {
    let expr = parse("1x + 2xy + 3");
    println!("Original expr: {}", expr);

    let simpl = simplify(&expr);

    println!("Expr simplified: {}", simpl);

    let foo = I8AddGroup;
    thing(&foo);
}

fn thing<G, S, E, O>(_: &G)
where
    S: Set<Elem = E>,
    O: ClosedBinOp<E> + Associative<E> + Identity<E> + Invertible<E> + Commutative<E>,
    G: AbelianGroup<S, E, O>,
{
}
