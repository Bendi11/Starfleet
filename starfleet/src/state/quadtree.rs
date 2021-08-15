//! A quadtree structure for efficiently storing system coordinates
use generational_arena::{Arena, Index};

use super::{Rect, Point};

/// The `Branch` struct is used in the [Branch](Node::Branch) variant of the [Node] enum,
/// and contains a bounding box for the contained nodes and the child nodes
///
/// The structure of child nodes is: 
/// ```_
///   _________
///  | NW | NE |
///  |_0__|_1__|
///  | SW | SE |
///  | 3  | 2  |
///   ^^^^^^^^^
/// ```
///
pub struct Branch {
    /// The bounding box of this branch
    bb: Rect,
    /// A branch always has at most 4 children
    children: Box<[Option<Node> ; 4]>,
}

/// A direction for the child nodes of a [Branch]
#[repr(u8)]
enum Dir {
    NW = 0,
    NE = 1,
    SE = 2,
    SW = 3
}


impl Dir {
    /// Return the given [direction](Dir) of this [Rect]
    #[inline]
    const fn of(&self, rect: Rect) -> Rect {
        match self {
            Self::NW => rect.nw(),
            Self::NE => rect.ne(),
            Self::SE => rect.se(),
            Self::SW => rect.sw()
        }
    }
}

/// One node in a [quad tree](QuadTree), either containing more children or 
/// a leaf node
pub enum Node {
    /// A branch in the tree, containing children nodes
    Branch(Branch),
    /// A leaf node with position and data
    Leaf((Point, Index)),
}

/// The `QuadTree` struct is used to hold a record of locations on a 2D coordinate grid
pub struct QuadTree<T> {
    /// Arena allocator we store all nodes in
    arena: Arena<T>,
    /// The root node of the quad tree
    root: Branch,
}

impl Node {
    /// Create a new [Branch](Node::Branch) variant with no children using the given bounding box
    pub fn branch(bb: Rect) -> Self {
        Self::Branch(Branch {
            bb,
            children: Box::new([ None, None, None, None ])
        })
    }

    /// Insert a handle to type `T` into this node, either filling an empty child node or 
    /// splitting this leaf into a branch
    ///
    /// Returns `true` if the value was inserted and `false` if insertion failed
    fn insert(&mut self, pos: Point, val: Index, area: Rect) -> bool {
        //Return false if we can't contain this point
        if !area.contains(pos) {
            return false
        }

        match self {
            //We will insert the node into one of our children
            Self::Branch(branch) => {
                //Tru to just push the new value to our children
                //if let Ok(()) = branch.children.try_push(Node::Leaf((pos, val))) {
                //    return true
                //}

                //Find the child node that this point should be in, manually unrolled loop here for optimization
                let nw = Dir::NW.of(area);
                if nw.contains(pos) {
                    match unsafe{ branch.children.get_unchecked_mut(Dir::NW as usize) } {
                        Some(node) => return node.insert(pos, val, nw),
                        node @ None => {
                            *node = Some(Node::Leaf((pos, val)));
                            true
                        }
                    }
                } else {
                    let sw = Dir::SW.of(area);
                    if sw.contains(pos) {
                        match unsafe{ branch.children.get_unchecked_mut(Dir::SW as usize) } {
                            Some(node) => return node.insert(pos, val, sw),
                            node @ None => {
                                *node = Some(Node::Leaf((pos, val)));
                                true
                            }
                        }
                    } else {
                        let se = Dir::SE.of(area);
                        if se.contains(pos) {
                            match unsafe{ branch.children.get_unchecked_mut(Dir::SE as usize) } {
                                Some(node) => return node.insert(pos, val, se),
                                node @ None => {
                                    *node = Some(Node::Leaf((pos, val)));
                                    true
                                }
                            }
                        } else {
                            let ne = Dir::NE.of(area);
                            if ne.contains(pos) {
                                match unsafe{ branch.children.get_unchecked_mut(Dir::NE as usize) } {
                                    Some(node) => return node.insert(pos, val, ne),
                                    node @ None => {
                                        *node = Some(Node::Leaf((pos, val)));
                                        true
                                    }
                                }
                            } else {
                                unreachable!("One of the child nodes must contain the point")
                            }
                        }
                    }
                }
            },
            //We need to split into quadrants
            Self::Leaf((old_point, old_handle)) => {
                let half_len = area.len() / 2;
                let half_height = area.height() / 2;

                /* Split into:
                    __________
                    | NW | NE |
                    |____|____|
                    | SW | SE |
                    |    |    |
                    ^^^^^^^^^^^
                */
                false
            }
        }
    }
}