//! A quadtree structure for efficiently storing system coordinates

use arrayvec::ArrayVec;

use super::Rect;

/// The `Branch` struct is used in the [Branch](Node::Branch) variant of the [Node] enum,
/// and contains a bounding box for the contained nodes and the child nodes
pub struct Branch<T: Sized> {
    /// The bounding box of this branch
    bb: Rect,
    /// A branch always has at most 4 children
    children: ArrayVec<Node<T>, 4>,
}

/// One node in a [quad tree](QuadTree), either containing more children or 
/// a leaf node
pub enum Node<T: Sized> {
    Branch(Box<[Node<T> ; 4]>),
    Leaf(T),
}

/// The `QuadTree` struct is used to hold a record of locations on a 2D coordinate grid
pub struct QuadTree<T: Sized> {
    root: Branch<T>
}

