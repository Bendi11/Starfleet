//! Structs for different units of measurement and methods to convert
//! between them

use serde::{Serialize, Deserialize};

macro_rules! impl_op {
    ($op:tt $name:ident $fn_name:ident $type:ty) => {
        impl ::std::ops::$name for $type {
            type Output = Self;
            fn $fn_name (self, other: Self) -> Self::Output {
                Self(self.0 $op other.0)
            }
        }
    };
    (assign $op:tt $name:ident $fn_name:ident $type:ty) => {
        impl ::std::ops::$name for $type {
            fn $fn_name (&mut self, other: Self) {
                *self = Self(self.0 $op other.0);
            }
        }
    };
}

/// A struct holding distance internally in meters
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Distance(f32);

impl_op!(+ Add add Distance);
impl_op!(- Sub sub Distance);
impl_op!(* Mul mul Distance);
impl_op!(/ Div div Distance);
impl_op!(% Rem rem Distance);
impl_op!(assign + AddAssign add_assign Distance);
impl_op!(assign - SubAssign sub_assign Distance);
impl_op!(assign * MulAssign mul_assign Distance);
impl_op!(assign / DivAssign div_assign Distance);
impl_op!(assign % RemAssign rem_assign Distance);

impl Distance {
    /// Get the distance as meters
    #[inline(always)]
    pub const fn meters(&self) -> f32 {
        self.0
    }
}