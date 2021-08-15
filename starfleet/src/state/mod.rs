//! The `state` module contains definitions for global state
//! contained in the engine

pub mod quadtree;

/// The `Point` struct stores position in a system or galaxy
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point(pub u16, pub u16);

impl Point {
    #[inline(always)]
    pub const fn x(&self) -> u16 {
        self.0
    }

    #[inline(always)]
    pub const fn y(&self) -> u16 {
        self.1
    }
}

macro_rules! impl_op {
    ($op:tt , $name:ident , $fn:ident) => {
        impl ::std::ops::$name for Point {
            type Output = Self;
            fn $fn (self, rhs: Self) -> Self::Output {
                Self(self.0 $op rhs.0, self.1 $op rhs.1)
            }
        }
    };
    ($op:tt, $name:ident, $fn:ident -assign) => {
        impl ::std::ops::$name for Point {
            fn $fn (&mut self, rhs: Self) {
                *self = Self(self.0 $op rhs.0, self.1 $op rhs.1);
            }
        }
    };
}

impl_op!(* , Mul , mul);
impl_op!(+, Add, add);
impl_op!(-, Sub, sub);
impl_op!(/, Div, div);
impl_op!(+, AddAssign, add_assign -assign);
impl_op!(-, SubAssign, sub_assign -assign);
impl_op!(*, MulAssign, mul_assign -assign);
impl_op!(/, DivAssign, div_assign -assign);

/// A rectangle made of a low corner point and a high corner point
/// ## Gurantees
/// The first [Point] must always be lower and further left than the second
/// [Point]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect(pub Point, pub Point);

impl Rect {
    /// Create a new `Rect` struct, with a debug_assert macro to ensure that `low` is always lower than `high`
    #[cfg(debug_assertions)]
    #[inline]
    pub fn new(low: Point, high: Point) -> Self {
        debug_assert!(low < high, "Attempting to construct a Rect struct with a higher low point");
        Self(low, high)
    }

    /// Create a new `Self` with no debug assertion
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    pub const fn new(low: Point, high: Point) -> Self {
        Self(low, high)
    }
    /// Get the area of this rectangle
    #[inline(always)]
    pub const fn area(&self) -> u32 {
        let len = self.0.x() - self.1.x();
        let height = self.0.y() - self.1.y();
        len as u32 * height as u32
    }

    /// Check if this rectangle contains a point
    pub const fn contains(&self, point: Point) -> bool {
        point.x() >= self.0.x() && point.y() >= self.1.y() && 
        point.x() <= self.1.x() && point.y() <= self.1.y()
    }
}


/// A star system contains any entities that are currently in the star system, and
/// is contained in the [Galaxy] struct
pub struct StarSystem {

}

/// The `Galaxy` struct tracks where each star system is
pub struct Galaxy {

}