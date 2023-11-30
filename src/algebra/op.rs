pub trait ClosedBinOp<T> {
    fn op(a: &T, b: &T) -> T;
}

pub trait Associative<T>: ClosedBinOp<T> {}
pub trait Identity<T>: ClosedBinOp<T> {
    fn identity() -> T;
}
pub trait LeftInvertible<T>: ClosedBinOp<T> {
    fn left_inverse(a: &T) -> T;
}
pub trait RightInvertible<T>: ClosedBinOp<T> {
    fn right_inverse(a: &T) -> T;
}
pub trait Invertible<T>: ClosedBinOp<T> + LeftInvertible<T> + RightInvertible<T> {
    // Left inverse and right inverse should be equal
    fn inverse(a: &T) -> T {
        Self::left_inverse(a)
    }
}
pub trait Commutative<T>: ClosedBinOp<T> {}

impl<T, O> Invertible<T> for O where O: ClosedBinOp<T> + LeftInvertible<T> + RightInvertible<T> {}
