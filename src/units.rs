use std::rc::Rc;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct Cell {
    pub x: i64,
    pub y: i64
}

pub type XCoord = i64;
pub type YCoord = i64;

/* QuadTree */
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    /*
      |A  B|
      |C  D|
    */

    // First bit says if a subtree has ON nodes (n), the last 7 bits tell the level (k)
    pub nk: u8,
    pub children: Option<[Rc<Node>; 4]>,
    pub hash: NodeHashType,
}

pub type NodeHashType = u64;

impl Node {
    #[inline]
    pub fn new(k: u8, abcd: [Rc<Node>; 4], n: bool, hash: u64) -> Rc<Node> {
        Rc::new(Node {
            nk: k | ((n as u8) << 7),
            children: Some(abcd),
            hash
        })
    }

    #[inline]
    pub fn new_empty(k: u8, n: bool, hash: u64) -> Rc<Node> {
        Rc::new(Node {
            nk: k | ((n as u8) << 7),
            hash,
            children: None,
        })
    }

    #[inline]
    pub fn n(&self) -> bool {
        self.nk & (1 << 7) != 0
    }

    #[inline]
    pub fn n_u8(&self) -> u8 {
        if self.nk & (1 << 7) != 0 {1} else {0}
    }

    #[inline]
    pub fn k(&self) -> u8 {
        self.nk & ((1 << 7) - 1)
    }

    #[inline]
    pub fn unwrap_children_cloned(&self) -> [Rc<Node>; 4] {
        self.children.clone().unwrap()
    }

}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}