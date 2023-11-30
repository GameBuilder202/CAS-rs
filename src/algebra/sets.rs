pub trait Contains {
    type Elem;
    fn contains(e: &Self::Elem) -> bool;
}
pub trait Subset<S, E>
where
    S: Set<Elem = E>,
{
    fn to_superset(e: &Self) -> S;
}
pub trait SuperSet<S, E>
where
    S: Set<Elem = E>,
{
    fn from_subset(e: &S) -> Self;
}
pub trait Union {
    fn union(&self, other: &Self) -> Self;
}
pub trait Intersection {
    fn intersection(&self, other: &Self) -> Self;
}
pub trait SetMinus {
    fn minus(&self, other: &Self) -> Self;
}
pub trait SymmetricDifference {
    fn symmdif(&self, other: &Self) -> Self;
}
pub trait Complement<S, E>: Subset<S, E>
where
    S: Set<Elem = E>,
{
    fn complement(&self, uni: &S) -> S;
}

impl<T> SetMinus for T
where
    T: Intersection + SymmetricDifference,
{
    fn minus(&self, other: &Self) -> Self {
        self.intersection(&self.symmdif(other))
    }
}

impl<T> SymmetricDifference for T
where
    T: SetMinus + Union,
{
    fn symmdif(&self, other: &Self) -> Self {
        self.minus(other).union(&other.minus(self))
    }
}

impl<T, S, E1, E2> SuperSet<S, E1> for T
where
    S: Set<Elem = E1> + Subset<T, E2>,
    T: Set<Elem = E2>,
{
    fn from_subset(e: &S) -> Self {
        S::to_superset(e)
    }
}
impl<T, S, E1, E2> Subset<S, E2> for T
where
    T: Set<Elem = E1> + SuperSet<S, E2>,
    S: Set<Elem = E2>,
{
    fn to_superset(e: &Self) -> S {
        S::from_subset(e)
    }
}

pub trait Set {
    type Elem;
}
impl<T, E> Set for T
where
    T: Contains<Elem = E> + Intersection + Union + SetMinus,
{
    type Elem = E;
}
