use std::fmt::Debug;
use std::iter::Product;
use std::ops::{Add, AddAssign, Div, DivAssign};

pub trait Num:
    Clone
    + Copy
    + Default
    + Debug
    + AddAssign
    + Add<Output = Self>
    + DivAssign
    + Div<Output = Self>
    + Product
{
    fn zero() -> Self {
        Self::default()
    }

    fn one() -> Self {
        Vec::<Self>::default().into_iter().product()
    }
}

impl Num for f64 {}
impl Num for f32 {}
impl Num for usize {}
impl Num for isize {}
impl Num for u128 {}
impl Num for i128 {}
impl Num for u64 {}
impl Num for i64 {}
impl Num for u32 {}
impl Num for i32 {}
impl Num for u16 {}
impl Num for i16 {}
impl Num for u8 {}
impl Num for i8 {}
