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

impl Branch {
    /// Insert the given point into the branch, returning `true` if the value was inserted
    fn insert(&mut self, pos: Point, val: Index) -> bool {
        if !self.bb.contains(pos) {
            return false
        }

        //Find the child node that this point should be in, manually unrolled loop here for optimization
        let nw = Dir::NW.of(self.bb);
        if nw.contains(pos) {
            match unsafe{ self.children.get_unchecked_mut(Dir::NW as usize) } {
                Some(node) => return node.insert(pos, val, nw),
                node @ None => {
                    *node = Some(Node::Leaf((pos, val)));
                    true
                }
            }
        } else {
            let sw = Dir::SW.of(self.bb);
            if sw.contains(pos) {
                match unsafe{ self.children.get_unchecked_mut(Dir::SW as usize) } {
                    Some(node) => return node.insert(pos, val, sw),
                    node @ None => {
                        *node = Some(Node::Leaf((pos, val)));
                        true
                    }
                }
            } else {
                let se = Dir::SE.of(self.bb);
                if se.contains(pos) {
                    match unsafe{ self.children.get_unchecked_mut(Dir::SE as usize) } {
                        Some(node) => return node.insert(pos, val, se),
                        node @ None => {
                            *node = Some(Node::Leaf((pos, val)));
                            true
                        }
                    }
                } else {
                    let ne = Dir::NE.of(self.bb);
                    if ne.contains(pos) {
                        match unsafe{ self.children.get_unchecked_mut(Dir::NE as usize) } {
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
    }

    /// Get the neighbors within a certain radius of a point
    fn neighbors(&self, pos: Point, radius: u16, neighbors: &mut Vec<(Point, Index)>) {
        let search_bb = Rect(
            Point(pos.x().saturating_sub(radius), pos.y().saturating_sub(radius)), 
            Point(pos.x().saturating_add(radius), pos.y().saturating_add(radius))
        );
        //Make sure this branch actually can contain a point in the search area
        if self.bb.intersects(search_bb) {
            //Search all child nodes for neighbors if we are in the search area
            for child in self.children.iter().flatten() {
                child.neighbors(pos, radius, neighbors)
            }
        }
    }
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
impl From<u8> for Dir {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NW,
            1 => Self::NE,
            2 => Self::SE,
            3 => Self::SW,
            _ => Self::NW
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
        match self {
            //We will insert the node into one of our children
            Self::Branch(branch) => branch.insert(pos, val),
            //We need to split into quadrants
            Self::Leaf((old_point, old_handle)) => {
                //Return false if we can't contain this point
                if !area.contains(pos) {
                    return false
                }
                let mut split = Self::branch(area);
                split.insert(*old_point, *old_handle, area); //Insert the old contained value of the leaf
                if split.insert(pos, val, area) {
                    *self = split;
                    true
                } else {
                    false
                }
                
            }
        }
    }

    /// Get all neighbors `radius` units from `pos`
    fn neighbors(&self, pos: Point, radius: u16, neighbors: &mut Vec<(Point, Index)>) {
        match self {
            Self::Branch(branch) => branch.neighbors(pos, radius, neighbors),
            Self::Leaf((leaf_pos, idx)) => {
                if leaf_pos.distance(pos) as u16 <= radius {
                    neighbors.push((*leaf_pos, *idx))
                }
            }
        }
    }
}

impl<T> QuadTree<T> {
    /// Return a new [QuadTree] with the maximum given bounds
    pub fn new(bounds: Rect) -> Self {
        Self {
            arena: Arena::new(),
            root: Branch { bb: bounds, children: Box::new([ None, None, None, None ]) }
        }
    }

    /// Insert a given value into the quad tree and return `Ok(())` if the point is able to be contained
    /// in this quad tree and was inserted, or `Err(val)` if it is not
    pub fn insert(&mut self, pos: Point, val: T) -> Result<(), T> {
        let handle = self.arena.insert(val);
        match self.root.insert(pos, handle) {
            true => Ok(()),
            false => Err(self.arena.remove(handle).unwrap())
        }
    }

    /// Get a list of all neighbors by searching in a circle around a point
    pub fn neighbors(&self, pos: Point, radius: u16) -> Vec<(Point, Index)> {
        let mut neighbors = Vec::new();
        self.root.neighbors(pos, radius, &mut neighbors); //Search root for neighbors
        neighbors
    }
}

use std::fmt;
impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NW => write!(f, "NW"),
            Self::NE => write!(f, "NE"),
            Self::SE => write!(f, "SE"),
            Self::SW => write!(f, "SW"),
        }
    }
}
impl<T: fmt::Debug> QuadTree<T> {
    /// Write the given branch to the formatter
    fn write_branch(&self, branch: &Branch, f: &mut fmt::Formatter<'_>, spaceno: u16) -> fmt::Result {
        for (dir, child) in branch.children.iter().enumerate() {
            for _ in 0..spaceno {
                write!(f, " ")?
            }
            write!(f, "[{}]{}: ", Dir::from(dir as u8).of(branch.bb), Dir::from(dir as u8))?;
            match child {
                Some(child) => match child {
                    Node::Branch(other) => {
                        writeln!(f)?;
                        self.write_branch(other, f, spaceno + 1)?
                    }
                    Node::Leaf((pos, data)) => {
                        write!(f, "{} [{:?}]", pos, self.arena[*data])?;
                    }
                },
                None => {
                    for _ in 0..spaceno {
                        write!(f, " ")?
                    }
                    write!(f, "<none>")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl<T: fmt::Display> QuadTree<T> {
    /// Write the given branch to the formatter
    fn write_branch_display(&self, branch: &Branch, f: &mut fmt::Formatter<'_>, spaceno: u16) -> fmt::Result {
        for (dir, child) in branch.children.iter().enumerate() {
            for _ in 0..spaceno {
                write!(f, " ")?
            }
            write!(f, "[{}]{}: ", Dir::from(dir as u8).of(branch.bb), Dir::from(dir as u8))?;
            match child {
                Some(child) => match child {
                    Node::Branch(other) => {
                        writeln!(f)?;
                        self.write_branch_display(other, f, spaceno + 1)?
                    }
                    Node::Leaf((pos, data)) => {
                        
                        write!(f, "{} [{}]", pos, self.arena[*data])?;
                    }
                },
                None => {
                    for _ in 0..spaceno {
                        write!(f, " ")?
                    }
                    write!(f, "<none>")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl<T: fmt::Debug> fmt::Debug for QuadTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_branch(&self.root, f, 0)
    }
}
impl<T: fmt::Display> fmt::Display for QuadTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_branch_display(&self.root, f, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_insert() {
        let mut quad = QuadTree::new(Rect::new(Point(0, 0), Point(100, 100)));
        assert_eq!(quad.insert(Point(0, 1), 100), Ok(()));
        quad.insert(Point(5, 1), 200).unwrap();
        quad.insert(Point(57, 57), 1231).unwrap();
        let neighbors = quad.neighbors(Point(2, 3), 5);
        let mut neighbors = neighbors.iter().map(|(point, _)| *point).collect::<Vec<Point>>();
        neighbors.sort();
        assert_eq!(neighbors, vec![
            Point(0, 1),
            Point(5, 1)
        ]);
    }
}