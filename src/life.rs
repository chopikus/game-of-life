use std::{collections::{BTreeMap, BTreeSet}, rc::Rc};

use super::units::{Cell, XCoord, YCoord, Node};
use std::cmp::{min, max};

pub struct Life {
   /* 
    Fixing user coordinates
    1. Add -min_x to shift_x, -min_y to shift_y, 
        so that all cells have coordinates >= (0, 0)
    2. Upon construction of a map, we centre() it until it gets big enough.
        However, (0,0) points after centering is at a different point,
        so all coordinates that user provides need to be fixed. 
    3. When calling get_alive_cells, 
        we add shift_x to min_x, max_x, 
        and add shift_y to min_y, max_y.

        Then when returning result,
        we subtract (shift_x,shift_y) from every cell we return.
   */
   shift_x: XCoord,
   shift_y: YCoord,
   root: Rc<Node>,
   on: Rc<Node>,
   off: Rc<Node>
}

impl Life {
    pub fn new(alive_cells: Vec<Cell>) -> Life {
        let off = Node::new_empty(0, false, 0);
        let on = Node::new_empty(0, true, 1);

        if alive_cells.is_empty() {
            return Life {
                on, off,
                shift_x: 0,
                shift_y: 0,
                root: off,
            };
        } else {
            /* alive_cells is not empty, can unwrap */
            let min_x: i64 = alive_cells.iter().map(|c| c.x).min().unwrap();
            let min_y: i64 = alive_cells.iter().map(|c| c.y).min().unwrap(); 
            
            let root: Rc<Node> = Self::construct_root(min_x, min_y, alive_cells);
            let shift_x: i64 = min_x;
            let shift_y: i64 = min_y;

            //while ()
            return Life {

            };
        }
    }

    fn construct_root(min_x: i64, min_y: i64, alive_cells: Vec<Cell>) -> Rc<Node> {
        // stub 
        Node::new_empty(0, false, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn glider() -> Life {
        let alive_cells = [
            Cell{x: 1, y: 0},
            Cell{x: 2, y: 1}, 
            Cell{x: 0, y: 2}, 
            Cell{x: 1, y: 2},
            Cell{x: 2, y: 2}
        ];

        Life::new(Vec::from(alive_cells))
    }

    #[test]
    fn start_cells() {
        let glider = glider();
        let alive_cells = glider.alive_cells();
        assert_eq!(alive_cells.len(), 5);
        assert!(alive_cells.contains(&Cell{x: 1, y: 0}));
        assert!(alive_cells.contains(&Cell{x: 2, y: 1}));
        assert!(alive_cells.contains(&Cell{x: 0, y: 2}));
        assert!(alive_cells.contains(&Cell{x: 1, y: 2}));
        assert!(alive_cells.contains(&Cell{x: 1, y: 2}));
    }

    #[test]
    fn tick_cells() {
        let mut glider = glider();
        glider.tick(0);

        let alive_cells = glider.alive_cells();
        assert_eq!(alive_cells.len(), 5);
        assert!(alive_cells.contains(&Cell{x: 0, y: 1}));
        assert!(alive_cells.contains(&Cell{x: 2, y: 1}));
        assert!(alive_cells.contains(&Cell{x: 1, y: 2}));
        assert!(alive_cells.contains(&Cell{x: 2, y: 2}));
        assert!(alive_cells.contains(&Cell{x: 1, y: 3}));
    }
}