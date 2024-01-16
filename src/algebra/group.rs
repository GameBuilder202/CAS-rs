use super::{op::*, sets::*};

pub trait Magma<S, E, O>
where
    S: Set<Elem = E>,
    O: ClosedBinOp<E>,
{
}
pub trait Semigroup<S, E, O>
where
    S: Set<Elem = E>,
    O: Associative<E>,
{
}
pub trait Monoid<S, E, O>
where
    S: Set<Elem = E>,
    O: Associative<E> + Identity<E>,
{
}
pub trait Group<S, E, O>
where
    S: Set<Elem = E>,
    O: Associative<E> + Identity<E> + Invertible<E>,
{
}
pub trait SubGroup<G, S1, S2, E, O>
where
    Self: Group<S1, E, O>,
    S1: Set<Elem = E> + Subset<S2, E>,
    S2: Set<Elem = E>,
    G: Group<S2, E, O>,
    O: Associative<E> + Identity<E> + Invertible<E>,
    G: Group<S2, E, O>,
{
}
pub trait AbelianGroup<S, E, O>
where
    S: Set<Elem = E>,
    O: Associative<E> + Identity<E> + Invertible<E> + Commutative<E>,
{
}

pub trait GroupHomomorphism<G1, S1, E1, O1, G2, S2, E2, O2>
where
    G1: Group<S1, E1, O1>,
    S1: Set<Elem = E1>,
    O1: Associative<E1> + Identity<E1> + Invertible<E1>,
    G2: Group<S2, E2, O2>,
    S2: Set<Elem = E2>,
    O2: Associative<E2> + Identity<E2> + Invertible<E2>,
{
    // &self isnt really needed but I need to keep it to impl on Fn(&E1) -> E2
    fn map(&self, e: &E1) -> E2;
}
pub trait GroupIsomorphism<G1, S1, E1, O1, G2, S2, E2, O2>
where
    G1: Group<S1, E1, O1>,
    S1: Set<Elem = E1>,
    O1: Associative<E1> + Identity<E1> + Invertible<E1>,
    G2: Group<S2, E2, O2>,
    S2: Set<Elem = E2>,
    O2: Associative<E2> + Identity<E2> + Invertible<E2>,
{
    fn map_to(&self, e: &E1) -> E2;
    fn map_from(&self, e: &E2) -> E1;
}

impl<S, E, O> Magma<S, E, O> for (S, O)
where
    S: Set<Elem = E>,
    O: ClosedBinOp<E>,
{
}
impl<S, E, O> Semigroup<S, E, O> for (S, O)
where
    S: Set<Elem = E>,
    O: Associative<E>,
{
}
impl<S, E, O> Monoid<S, E, O> for (S, O)
where
    S: Set<Elem = E>,
    O: Associative<E> + Identity<E>,
{
}
impl<S, E, O> Group<S, E, O> for (S, O)
where
    S: Set<Elem = E>,
    O: Associative<E> + Identity<E> + Invertible<E>,
{
}
impl<G1, G2, S1, S2, E, O> SubGroup<G2, S1, S2, E, O> for G1
where
    G1: Group<S1, E, O>,
    S1: Set<Elem = E> + Subset<S2, E>,
    S2: Set<Elem = E>,
    O: Associative<E> + Identity<E> + Invertible<E>,
    G2: Group<S2, E, O>,
{
}
impl<S, E, O> AbelianGroup<S, E, O> for (S, O)
where
    S: Set<Elem = E>,
    O: Associative<E> + Identity<E> + Invertible<E> + Commutative<E>,
{
}

// grrr
// impl<S, E, O, M> Set for M
// where
//     M: Magma<S, E, O>,
//     S: Set<Elem = E>,
//     O: ClosedBinOp<E>,
// {
//     type Elem = E;

//     fn contains(e: &Self::Elem) -> bool {
//         S::contains(e)
//     }
// }

// Convenience macros because the traits have so many generics
#[macro_export]
macro_rules! impl_group_homomorphism {
    ($F:ident $({$bound:path})? (&$self:ident, $e:ident) -> $body:block) => {
        impl<G1, S1, E1, O1, G2, S2, E2, O2, F> GroupHomomorphism<G1, S1, E1, O1, G2, S2, E2, O2>
            for $F
        where
            G1: Group<S1, E1, O1>,
            S1: Set<Elem = E1>,
            O1: Associative<E1> + Identity<E1> + Invertible<E1>,
            G2: Group<S2, E2, O2>,
            S2: Set<Elem = E2>,
            O2: Associative<E2> + Identity<E2> + Invertible<E2>,
            $($F: $bound)?,
        {
            fn map(&$self, $e: &E1) -> E2 $body
        }
    };
}
#[macro_export]
macro_rules! impl_group_isomorphism {
    ($F:ident $({$bound:path})? (&$self:ident, $e:ident) -> to $body1:block; from $body2:block) => {
        impl<G1, S1, E1, O1, G2, S2, E2, O2, F> GroupIsomorphism<G1, S1, E1, O1, G2, S2, E2, O2>
            for $F
        where
            G1: Group<S1, E1, O1>,
            S1: Set<Elem = E1>,
            O1: Associative<E1> + Identity<E1> + Invertible<E1>,
            G2: Group<S2, E2, O2>,
            S2: Set<Elem = E2>,
            O2: Associative<E2> + Identity<E2> + Invertible<E2>,
            $($F: $bound)?,
        {
            fn map_to(&$self, $e: &E1) -> E2 $body1
            fn map_from(&$self, $e: &E2) -> E1 $body2
        }
    };
}

// Not all functions from a type to another are group homomorphisms, this was just a test to make sure the macro works
// impl_group_homomorphism!(F {Fn(&E1) -> E2} (&self, e) -> {
//     self(e)
// });

impl_group_homomorphism!(F {GroupIsomorphism<G1, S1, E1, O1, G2, S2, E2, O2>} (&self, e) -> { self.map_to(e) });
// grr this conflicts with automorphisms
// impl_group_isomorphism! (F {GroupIsomorphism<G2, S2, E2, O2, G1, S1, E1, O1>} (&self, e) -> to { self.map_from(e) }; from { self.map_to(e) });
