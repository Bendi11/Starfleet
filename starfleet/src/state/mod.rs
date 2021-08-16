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

    /// Return the distance between this point and another point
    pub fn distance(&self, other: Self) -> f32 {
        ( ((other.0.max(self.0).saturating_sub(other.0.min(self.0))).pow(2) as f32)
             + 
        ((other.1.max(self.1).saturating_sub(other.1.min(self.1))).pow(2) as f32) )
        .sqrt()
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

    /// Get the lowest and leftmost point
    #[inline(always)]
    pub const fn low(&self) -> Point {
        self.0
    }

    /// Get the highest and rightmost point
    #[inline(always)]
    pub const fn high(&self) -> Point {
        self.1
    }


    /// Get the north western quarter of this rectangle
    pub const fn nw(&self) -> Rect {
        Rect(
            Point(self.low().x(), self.low().y() + (self.height() / 2)), 
            Point(self.high().x() - (self.len() / 2), self.high().y())
        )
    }

    /// Get the north eastern quarter of this rectangle
    pub const fn ne(&self) -> Rect {
        Rect(
            self.center(), 
            self.high()
        )
    }

    /// Get the south eastern quarter of this rectangle
    pub const fn se(&self) -> Rect {
        let center = self.center();
        let half_height = self.height() / 2;
        Rect(
            Point(center.x(), center.y() - half_height), 
            Point(self.high().x(), self.high().y() - half_height)
        )
    }

    /// Get the south western quarter of this rectangle
    pub const fn sw(&self) -> Rect {
        Rect(
            self.low(), 
            self.center()
        )
    }


    /// Return the center of this rectangle
    pub const fn center(&self) -> Point {
        Point(self.low().x() + (self.len() / 2), self.low().y() + (self.height() / 2))
    }

    /// Get the length of this rectangle
    #[inline(always)]
    pub const fn len(&self) -> u16 {
        self.1.x() - self.0.x()
    }

    /// Get the height of this rectangle
    #[inline(always)]
    pub const fn height(&self) -> u16 {
        self.1.y() - self.0.y()
    }

    /// Check if this rectangle contains a point
    pub const fn contains(&self, point: Point) -> bool {
        point.x() >= self.low().x() && point.y() >= self.low().y() && 
        point.x() <= self.high().x() && point.y() <= self.high().y()
    }

    /// Check if one [Rect] intersects with another
    pub const fn intersects(&self, other: Rect) -> bool {
        self.contains(other.0) || self.contains(other.1)
    }
}
use std::fmt;

use indexmap::IndexMap;
use legion::Entity;

use self::quadtree::QuadTree;
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.0, self.1)
    }
}

/// A star system contains any entities that are currently in the star system, and
/// is contained in the [Galaxy] struct
pub struct StarSystem {
    /// A map of entities to their locations
    entities: QuadTree<Entity>,
}

/// The `Galaxy` struct tracks where all star systems are in the game
pub struct Galaxy {
    /// A virtual map of star system indexes in the `star_map` hashmap
    stars: QuadTree<usize>,
    /// A map of star system names to star system data
    star_map: IndexMap<String, StarSystem>,
}