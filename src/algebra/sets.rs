use std::marker::PhantomData;

pub trait Subset<S, E>
where
    S: Set<Elem = E>,
{
}

pub struct Union<S1, S2, E>(PhantomData<(S1, S2)>)
where
    S1: Set<Elem = E>,
    S2: Set<Elem = E>;
pub struct Intersection<S1, S2, E>(PhantomData<(S1, S2)>)
where
    S1: Set<Elem = E>,
    S2: Set<Elem = E>;
pub struct SetMinus<S1, S2, E>(PhantomData<(S1, S2)>)
where
    S1: Set<Elem = E>,
    S2: Set<Elem = E>;
pub struct SymmetricDifference<S1, S2, E>(PhantomData<(S1, S2)>)
where
    S1: Set<Elem = E>,
    S2: Set<Elem = E>;
pub struct Complement<U, S, E>(PhantomData<(U, S)>)
where
    U: Set<Elem = E>,
    S: Set<Elem = E> + Subset<U, E>;
pub struct CartesianProd<S1, E1, S2, E2>(PhantomData<(S1, S2)>)
where
    S1: Set<Elem = E1>,
    S2: Set<Elem = E2>;

pub trait Set: Sized {
    type Elem;

    fn contains(e: &Self::Elem) -> bool;

    fn union<S: Set<Elem = Self::Elem>>() -> Union<Self, S, Self::Elem>
    where
        Self: Sized,
    {
        Union(PhantomData)
    }
    fn intersection<S: Set<Elem = Self::Elem>>() -> Intersection<Self, S, Self::Elem>
    where
        Self: Sized,
    {
        Intersection(PhantomData)
    }
    fn minus<S: Set<Elem = Self::Elem>>() -> SetMinus<Self, S, Self::Elem>
    where
        Self: Sized,
    {
        SetMinus(PhantomData)
    }
    fn symmdif<S: Set<Elem = Self::Elem>>() -> SymmetricDifference<Self, S, Self::Elem>
    where
        Self: Sized,
    {
        SymmetricDifference(PhantomData)
    }
    fn complement<U>() -> Complement<U, Self, Self::Elem>
    where
        Self: Sized + Subset<U, Self::Elem>,
        U: Set<Elem = Self::Elem>,
    {
        Complement(PhantomData)
    }
    fn cartesian_prod<S, E>() -> CartesianProd<Self, Self::Elem, S, E>
    where
        S: Set<Elem = E>,
    {
        CartesianProd(PhantomData)
    }
}

impl<S1, S2, E> Set for Union<S1, S2, E>
where
    S1: Set<Elem = E>,
    S2: Set<Elem = E>,
{
    type Elem = E;

    fn contains(e: &Self::Elem) -> bool {
        S1::contains(e) || S2::contains(e)
    }
}
impl<S1, S2, E> Set for Intersection<S1, S2, E>
where
    S1: Set<Elem = E>,
    S2: Set<Elem = E>,
{
    type Elem = E;

    fn contains(e: &Self::Elem) -> bool {
        S1::contains(e) && S2::contains(e)
    }
}
impl<S1, S2, E> Set for SetMinus<S1, S2, E>
where
    S1: Set<Elem = E>,
    S2: Set<Elem = E>,
{
    type Elem = E;

    fn contains(e: &Self::Elem) -> bool {
        S1::contains(e) && !S2::contains(e)
    }
}
impl<S1, S2, E> Set for SymmetricDifference<S1, S2, E>
where
    S1: Set<Elem = E>,
    S2: Set<Elem = E>,
{
    type Elem = E;

    fn contains(e: &Self::Elem) -> bool {
        S1::contains(e) ^ S2::contains(e)
    }
}
impl<U, S, E> Set for Complement<U, S, E>
where
    U: Set<Elem = E>,
    S: Set<Elem = E> + Subset<U, E>,
{
    type Elem = E;

    fn contains(e: &Self::Elem) -> bool {
        U::contains(e) && !S::contains(e)
    }
}
impl<S1, E1, S2, E2> Set for CartesianProd<S1, E1, S2, E2>
where
    S1: Set<Elem = E1>,
    S2: Set<Elem = E2>,
{
    type Elem = (E1, E2);

    fn contains(e: &Self::Elem) -> bool {
        S1::contains(&e.0) && S2::contains(&e.1)
    }
}

// LET ME DO THIS AAA
// impl<F, A, B> Set for F
// where
//     F: Fn(A) -> B,
//     A: Sized,
//     B: Sized,
// {
//     type Elem = (A, B);
// }
