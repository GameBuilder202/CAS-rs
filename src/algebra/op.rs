use std::ops::{Add, Mul};

pub trait ClosedBinOp<T> {
    fn op(a: &T, b: &T) -> T;
}

pub trait Associative<T>: ClosedBinOp<T> {}
pub trait Identity<T>: ClosedBinOp<T> {
    fn identity() -> T;
}
pub trait Invertible<T>: ClosedBinOp<T> {
    // Left inverse and right inverse should be equal
    fn inverse(a: &T) -> T;
}
pub trait Commutative<T>: ClosedBinOp<T> {}

pub trait AddOp<T>: ClosedBinOp<T>
where
    T: Add,
{
}
pub trait MulOp<T>: ClosedBinOp<T>
where
    T: Mul,
{
}
