#![allow(non_upper_case_globals)]

pub mod group;
pub mod op;
pub mod ring;
pub mod sets;

use op::*;
use sets::*;

// Some concrete sets for common datatypes
macro_rules! DTSet {
    ($name:ident -> $typ:ty) => {
        pub struct $name;
        impl Set for $name {
            type Elem = $typ;
            fn contains(_: &Self::Elem) -> bool {
                true
            }
        }
    };

    ($name1:ident -> $typ1:ty, $($name2:ident -> $typ2:ty),+) => {
        DTSet!($name1 -> $typ1);
        $(DTSet!($name2 -> $typ2);)+
    }
}

DTSet!(U8Set -> u8, U16Set -> u16, U32Set -> u32, U64Set -> u64, U128Set -> u128, USizeSet -> usize);
DTSet!(I8Set -> i8, I16Set -> i16, I32Set -> i32, I64Set -> i64, I128Set -> i128, ISizeSet -> isize);
DTSet!(F32Set -> f32, F64Set -> f64);

// Add and Mult operators for the common datatypes
macro_rules! DTAddOp {
    ($name:ident -> $typ:ty) => {
        pub struct $name;
        impl ClosedBinOp<$typ> for $name {
            fn op(a: &$typ, b: &$typ) -> $typ {
                a + b
            }
        }
        impl Associative<$typ> for $name {}
        impl Identity<$typ> for $name {
            fn identity() -> $typ {
                0 as $typ
            }
        }
        impl Commutative<$typ> for $name {}
    };

    ($name1:ident -> $typ1:ty, $($name2:ident -> $typ2:ty),+) => {
        DTAddOp!($name1 -> $typ1);
        $(DTAddOp!($name2 -> $typ2);)+
    }}

DTAddOp!(U8AddOp -> u8, U16AddOp -> u16, U32AddOp -> u32, U64AddOp -> u64, U128AddOp -> u128, USizeAddOp -> usize);
DTAddOp!(I8AddOp -> i8, I16AddOp -> i16, I32AddOp -> i32, I64AddOp -> i64, I128AddOp -> i128, ISizeAddOp -> isize);
impl Invertible<i8> for I8AddOp {
    fn inverse(a: &i8) -> i8 {
        -a
    }
}
impl Invertible<i16> for I16AddOp {
    fn inverse(a: &i16) -> i16 {
        -a
    }
}
impl Invertible<i32> for I32AddOp {
    fn inverse(a: &i32) -> i32 {
        -a
    }
}
impl Invertible<i64> for I64AddOp {
    fn inverse(a: &i64) -> i64 {
        -a
    }
}
impl Invertible<i128> for I128AddOp {
    fn inverse(a: &i128) -> i128 {
        -a
    }
}
impl Invertible<isize> for ISizeAddOp {
    fn inverse(a: &isize) -> isize {
        -a
    }
}

// Ok well yes floats are not commutative + associative but... cmon...
DTAddOp!(F32AddOp -> f32, F64AddOp -> f64);
impl Invertible<f32> for F32AddOp {
    fn inverse(a: &f32) -> f32 {
        -a
    }
}
impl Invertible<f64> for F64AddOp {
    fn inverse(a: &f64) -> f64 {
        -a
    }
}

macro_rules! DTMulOp {
    ($name:ident -> $typ:ty) => {
        pub struct $name;
        impl ClosedBinOp<$typ> for $name {
            fn op(a: &$typ, b: &$typ) -> $typ {
                a + b
            }
        }
        impl Associative<$typ> for $name {}
        impl Identity<$typ> for $name {
            fn identity() -> $typ {
                1 as $typ
            }
        }
        impl Commutative<$typ> for $name {}
    };

    ($name1:ident -> $typ1:ty, $($name2:ident -> $typ2:ty),+) => {
        DTMulOp!($name1 -> $typ1);
        $(DTMulOp!($name2 -> $typ2);)+
    }}

DTMulOp!(U8MulOp -> u8, U16MulOp -> u16, U32MulOp -> u32, U64MulOp -> u64, U128MulOp -> u128, USizeMulOp -> usize);
DTMulOp!(I8MulOp -> i8, I16MulOp -> i16, I32MulOp -> i32, I64MulOp -> i64, I128MulOp -> i128, ISizeMulOp -> isize);
// Ok well yes floats are not commutative + associative but... cmon...
DTMulOp!(F32MulOp -> f32, F64MulOp -> f64);
impl Invertible<f32> for F32MulOp {
    fn inverse(a: &f32) -> f32 {
        1f32 / a
    }
}
impl Invertible<f64> for F64MulOp {
    fn inverse(a: &f64) -> f64 {
        1f64 / a
    }
}

// Algebraic structures for common datatypes, the appropriate traits should be impl-ed automatically

// (S, +)
pub const U8AddMonoid: (U8Set, U8AddOp) = (U8Set, U8AddOp);
pub const U16AddMonoid: (U16Set, U16AddOp) = (U16Set, U16AddOp);
pub const U32AddMonoid: (U32Set, U32AddOp) = (U32Set, U32AddOp);
pub const U64AddMonoid: (U64Set, U64AddOp) = (U64Set, U64AddOp);
pub const U128AddMonoid: (U128Set, U128AddOp) = (U128Set, U128AddOp);
pub const USizeAddMonoid: (USizeSet, USizeAddOp) = (USizeSet, USizeAddOp);

pub const I8AddGroup: (I8Set, I8AddOp) = (I8Set, I8AddOp);
pub const I16AddGroup: (I16Set, I16AddOp) = (I16Set, I16AddOp);
pub const I32AddGroup: (I32Set, I32AddOp) = (I32Set, I32AddOp);
pub const I64AddGroup: (I64Set, I64AddOp) = (I64Set, I64AddOp);
pub const I128AddGroup: (I128Set, I128AddOp) = (I128Set, I128AddOp);
pub const ISizeAddGroup: (ISizeSet, ISizeAddOp) = (ISizeSet, ISizeAddOp);

pub const F32AddGroup: (F32Set, F32AddOp) = (F32Set, F32AddOp);
pub const F64AddGroup: (F64Set, F64AddOp) = (F64Set, F64AddOp);

// (S, *)
pub const U8MulMonoid: (U8Set, U8MulOp) = (U8Set, U8MulOp);
pub const U16MulMonoid: (U16Set, U16MulOp) = (U16Set, U16MulOp);
pub const U32MulMonoid: (U32Set, U32MulOp) = (U32Set, U32MulOp);
pub const U64MulMonoid: (U64Set, U64MulOp) = (U64Set, U64MulOp);
pub const U128MulMonoid: (U128Set, U128MulOp) = (U128Set, U128MulOp);
pub const USizeMulMonoid: (USizeSet, USizeMulOp) = (USizeSet, USizeMulOp);

pub const I8MulMonoid: (I8Set, I8MulOp) = (I8Set, I8MulOp);
pub const I16MulMonoid: (I16Set, I16MulOp) = (I16Set, I16MulOp);
pub const I32MulMonoid: (I32Set, I32MulOp) = (I32Set, I32MulOp);
pub const I64MulMonoid: (I64Set, I64MulOp) = (I64Set, I64MulOp);
pub const I128MulMonoid: (I128Set, I128MulOp) = (I128Set, I128MulOp);
pub const ISizeMulMonoid: (ISizeSet, ISizeMulOp) = (ISizeSet, ISizeMulOp);

pub const F32MulGroup: (F32Set, F32MulOp) = (F32Set, F32MulOp);
pub const F64MulGroup: (F64Set, F64MulOp) = (F64Set, F64MulOp);

// (S, +, *)
pub const U8SemiRing: (U8Set, U8AddOp, U8MulOp) = (U8Set, U8AddOp, U8MulOp);
pub const U16SemiRing: (U16Set, U16AddOp, U16MulOp) = (U16Set, U16AddOp, U16MulOp);
pub const U32SemiRing: (U32Set, U32AddOp, U32MulOp) = (U32Set, U32AddOp, U32MulOp);
pub const U64SemiRing: (U64Set, U64AddOp, U64MulOp) = (U64Set, U64AddOp, U64MulOp);
pub const U128SemiRing: (U128Set, U128AddOp, U128MulOp) = (U128Set, U128AddOp, U128MulOp);
pub const USizeSemiRing: (USizeSet, USizeAddOp, USizeMulOp) = (USizeSet, USizeAddOp, USizeMulOp);

pub const I8Ring: (I8Set, I8AddOp, I8MulOp) = (I8Set, I8AddOp, I8MulOp);
pub const I16Ring: (I16Set, I16AddOp, I16MulOp) = (I16Set, I16AddOp, I16MulOp);
pub const I32Ring: (I32Set, I32AddOp, I32MulOp) = (I32Set, I32AddOp, I32MulOp);
pub const I64Ring: (I64Set, I64AddOp, I64MulOp) = (I64Set, I64AddOp, I64MulOp);
pub const I128Ring: (I128Set, I128AddOp, I128MulOp) = (I128Set, I128AddOp, I128MulOp);
pub const ISizeRing: (ISizeSet, ISizeAddOp, ISizeMulOp) = (ISizeSet, ISizeAddOp, ISizeMulOp);

pub const F32Field: (F32Set, F32AddOp, F32MulOp) = (F32Set, F32AddOp, F32MulOp);
pub const F64Field: (F64Set, F64AddOp, F64MulOp) = (F64Set, F64AddOp, F64MulOp);
