use std::ops::{Add, Mul};

use super::{group::*, op::*, sets::*};

// Just hope the multiplication operator distributes over addition because I cant figure out how to make a trait for that

pub trait Semiring<S, E, A, M>
where
    S: Set<Elem = E>,
    E: Add + Mul,
    (S, A): Monoid<S, E, A>, /* More specifically a commutative monoid as mentioned by the trait bounds below */
    (S, M): Monoid<S, E, M>,
    A: AddOp<E> + Associative<E> + Identity<E> + Commutative<E>,
    M: MulOp<E> + Associative<E> + Identity<E>,
{
}
pub trait Rng<S, E, A, M>
where
    S: Set<Elem = E>,
    E: Add + Mul,
    (S, A): AbelianGroup<S, E, A>,
    (S, M): Semigroup<S, E, M>,
    A: AddOp<E> + Associative<E> + Identity<E> + Invertible<E> + Commutative<E>,
    M: MulOp<E> + Associative<E>,
{
}
pub trait Ring<S, E, A, M>
where
    S: Set<Elem = E>,
    E: Add + Mul,
    (S, A): AbelianGroup<S, E, A>,
    (S, M): Monoid<S, E, M>,
    A: AddOp<E> + Associative<E> + Identity<E> + Invertible<E> + Commutative<E>,
    M: MulOp<E> + Associative<E> + Identity<E>,
{
}
pub trait SubRing<R, S1, S2, E, A, M>
where
    Self: Ring<S1, E, A, M>,
    S1: Set<Elem = E> + Subset<S2, E>,
    S2: Set<Elem = E>,
    E: Add + Mul,
    A: AddOp<E> + Associative<E> + Identity<E> + Invertible<E> + Commutative<E>,
    M: MulOp<E> + Associative<E> + Identity<E>,
    R: Ring<S2, E, A, M>,
{
}
pub trait CommutativeRing<S, E, A, M>
where
    S: Set<Elem = E>,
    E: Add + Mul,
    (S, A): AbelianGroup<S, E, A>,
    (S, M): Monoid<S, E, M>, /* More specifically a commutative monoid as mentioned by the trait bounds below */
    A: AddOp<E> + Associative<E> + Identity<E> + Invertible<E> + Commutative<E>,
    M: MulOp<E> + Associative<E> + Identity<E> + Commutative<E>,
{
}

pub trait RingHomomorphism<R1, S1, E1, A1, M1, R2, S2, E2, A2, M2>
where
    R1: Ring<S1, E1, A1, M1>,
    S1: Set<Elem = E1>,
    E1: Add + Mul,
    // (S1, A1): AbelianGroup<S1, E1, A1>, // wtf this isnt needed???
    A1: AddOp<E1> + Associative<E1> + Identity<E1> + Invertible<E1> + Commutative<E1>,
    M1: MulOp<E1> + Associative<E1> + Identity<E1>,
    R2: Ring<S2, E2, A2, M2>,
    S2: Set<Elem = E2>,
    E2: Add + Mul,
    A2: AddOp<E2> + Associative<E2> + Identity<E2> + Invertible<E2> + Commutative<E2>,
    M2: MulOp<E2> + Associative<E2> + Identity<E2>,
{
    // &self isnt really needed but I need to keep it to impl on Fn(&E1) -> E2
    fn map(&self, e: &E1) -> E2;
}
pub trait RingIsomorphism<R1, S1, E1, A1, M1, R2, S2, E2, A2, M2>
where
    R1: Ring<S1, E1, A1, M1>,
    S1: Set<Elem = E1>,
    E1: Add + Mul,
    A1: AddOp<E1> + Associative<E1> + Identity<E1> + Invertible<E1> + Commutative<E1>,
    M1: MulOp<E1> + Associative<E1> + Identity<E1>,
    R2: Ring<S2, E2, A2, M2>,
    S2: Set<Elem = E2>,
    E2: Add + Mul,
    A2: AddOp<E2> + Associative<E2> + Identity<E2> + Invertible<E2> + Commutative<E2>,
    M2: MulOp<E2> + Associative<E2> + Identity<E2>,
{
    fn map_to(&self, e: &E1) -> E2;
    fn map_from(&self, e: &E2) -> E1;
}

impl<S, E, A, M> Semiring<S, E, A, M> for (S, A, M)
where
    S: Set<Elem = E>,
    E: Add + Mul,
    (S, A): Monoid<S, E, A>,
    (S, M): Monoid<S, E, M>,
    A: AddOp<E> + Associative<E> + Identity<E> + Commutative<E>,
    M: MulOp<E> + Associative<E> + Identity<E>,
{
}
impl<S, E, A, M> Rng<S, E, A, M> for (S, A, M)
where
    S: Set<Elem = E>,
    E: Add + Mul,
    (S, A): AbelianGroup<S, E, A>,
    (S, M): Semigroup<S, E, M>,
    A: AddOp<E> + Associative<E> + Identity<E> + Invertible<E> + Commutative<E>,
    M: MulOp<E> + Associative<E>,
{
}
impl<S, E, A, M> Ring<S, E, A, M> for (S, A, M)
where
    S: Set<Elem = E>,
    E: Add + Mul,
    (S, A): AbelianGroup<S, E, A>,
    (S, M): Monoid<S, E, M>,
    A: AddOp<E> + Associative<E> + Identity<E> + Invertible<E> + Commutative<E>,
    M: MulOp<E> + Associative<E> + Identity<E>,
{
}
impl<R1, R2, S1, S2, E, A, M> SubRing<R2, S1, S2, E, A, M> for R1
where
    R1: Ring<S1, E, A, M>,
    S1: Set<Elem = E> + Subset<S2, E>,
    S2: Set<Elem = E>,
    E: Add + Mul,
    A: AddOp<E> + Associative<E> + Identity<E> + Invertible<E> + Commutative<E>,
    M: MulOp<E> + Associative<E> + Identity<E>,
    R2: Ring<S2, E, A, M>,
{
}
impl<S, E, A, M> CommutativeRing<S, E, A, M> for (S, A, M)
where
    S: Set<Elem = E>,
    E: Add + Mul,
    (S, A): AbelianGroup<S, E, A>,
    (S, M): Monoid<S, E, M>,
    A: AddOp<E> + Associative<E> + Identity<E> + Invertible<E> + Commutative<E>,
    M: MulOp<E> + Associative<E> + Identity<E> + Commutative<E>,
{
}

#[macro_export]
macro_rules! impl_ring_homomorphism {
    ($F:ident $({$bound:path})? (&$self:ident, $e:ident) -> $body:block) => {
        impl<R1, S1, E1, A1, M1, R2, S2, E2, A2, M2, F> RingHomomorphism<R1, S1, E1, A1, M1, R2, S2, E2, A2, M2>
            for $F
        where
        R1: Ring<S1, E1, A1, M1>,
        S1: Set<Elem = E1>,
        E1: Add + Mul,
        A1: AddOp<E1> + Associative<E1> + Identity<E1> + Invertible<E1> + Commutative<E1>,
        M1: MulOp<E1> + Associative<E1> + Identity<E1>,
        R2: Ring<S2, E2, A2, M2>,
        S2: Set<Elem = E2>,
        E2: Add + Mul,
        A2: AddOp<E2> + Associative<E2> + Identity<E2> + Invertible<E2> + Commutative<E2>,
        M2: MulOp<E2> + Associative<E2> + Identity<E2>,
        $($F: $bound)?,
        {
            fn map(&$self, $e: &E1) -> E2 $body
        }
    };
}
#[macro_export]
macro_rules! impl_ring_isomorphism {
    ($F:ident $({$bound:path})? (&$self:ident, $e:ident) -> to $body1:block; from $body2:block) => {
        impl<R1, S1, E1, A1, M1, R2, S2, E2, A2, M2, F> RingIsomorphism<R1, S1, E1, A1, M1, R2, S2, E2, A2, M2>
            for $F
        where
        R1: Ring<S1, E1, A1, M1>,
        S1: Set<Elem = E1>,
        E1: Add + Mul,
        A1: AddOp<E1> + Associative<E1> + Identity<E1> + Invertible<E1> + Commutative<E1>,
        M1: MulOp<E1> + Associative<E1> + Identity<E1>,
        R2: Ring<S2, E2, A2, M2>,
        S2: Set<Elem = E2>,
        E2: Add + Mul,
        A2: AddOp<E2> + Associative<E2> + Identity<E2> + Invertible<E2> + Commutative<E2>,
        M2: MulOp<E2> + Associative<E2> + Identity<E2>,
        $($F: $bound)?,
        {
            fn map_to(&$self, $e: &E1) -> E2 $body1
            fn map_from(&$self, $e: &E2) -> E1 $body2
        }
    };
}

// Not all functions from a type to another are ring homomorphisms, this was just a test to make sure the macro works
// impl_ring_homomorphism!(F {Fn(&E1) -> E2} (&self, e) -> {
//     self(e)
// });

impl_ring_homomorphism!(F {RingIsomorphism<R1, S1, E1, A1, M1, R2, S2, E2, A2, M2>} (&self, e) -> { self.map_to(e) });
impl_ring_isomorphism! (F {RingIsomorphism<R2, S2, E2, A2, M2, R1, S1, E1, A1, M1>} (&self, e) -> to { self.map_from(e) }; from { self.map_to(e) });
