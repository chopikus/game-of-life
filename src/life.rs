use std::collections::{BTreeMap, HashMap};
use std::hash::BuildHasherDefault;
use std::num::NonZeroUsize;
use std::rc::Rc;

use lru::LruCache;
use nohash_hasher::NoHashHasher;
use seahash::SeaHasher;
use crate::units::NodeHashType;

use super::units::{Cell, XCoord, YCoord, Node};

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
   off: Rc<Node>,

   join_cache: JoinCacheType,
   zero_cache: ZeroCacheType,
   life_4x4_cache: Life4x4CacheType
}

type DefaultSeaHasher = BuildHasherDefault<SeaHasher>;
type JoinCacheType = LruCache<[Rc<Node>; 4], Rc<Node>, DefaultSeaHasher>; 

type ZeroCacheType = BTreeMap<u8, Rc<Node>>;
pub type NodeHasher = BuildHasherDefault<NoHashHasher<NodeHashType>>;
type Life4x4CacheType = HashMap<Rc<Node>, Rc<Node>, NodeHasher>;

impl Life {
    fn life() -> Life {
       let join_cache_cap = NonZeroUsize::new(6000000).unwrap();
       let join_cache_hasher = BuildHasherDefault::<SeaHasher>::default();
       let join_cache: JoinCacheType = LruCache::with_hasher(join_cache_cap, join_cache_hasher);

       let zero_cache: ZeroCacheType = BTreeMap::new();
       let life_4x4_cache: Life4x4CacheType = HashMap::default();

       let off = Node::new_empty(0, false, 0);
       let on = Node::new_empty(0, true, 1);

       return Life {
           on: Rc::clone(&on),
           off: Rc::clone(&off),
           shift_x: 0,
           shift_y: 0,
           root: Rc::clone(&off),
           join_cache,
           zero_cache,
           life_4x4_cache
       };
    }

    pub fn new(alive_cells: Vec<Cell>) -> Life {
        let mut life = Self::life();
        
        if alive_cells.is_empty() {
            return life;
        } else {
            /* alive_cells is not empty, can unwrap */
            let min_x: i64 = alive_cells.iter().map(|c| c.x).min().unwrap();
            let min_y: i64 = alive_cells.iter().map(|c| c.y).min().unwrap(); 
            
            life.root = life.construct_root(min_x, min_y, alive_cells);
            life.shift_x = min_x;
            life.shift_y = min_y;
            
            while life.root.k() < 29 {
                life.shift_x += 1 << (life.root.k() - 1);
                life.shift_y += 1 << (life.root.k() - 1);

                life.root = life.centre(Rc::clone(&life.root));
            }

            return life;
        }
    }

    fn construct_root(&mut self, min_x: i64, min_y: i64, alive_cells: Vec<Cell>) -> Rc<Node> {
        let mut pattern = alive_cells.into_iter()
                          .map(|cell| (
                                       (cell.x - min_x, cell.y - min_y),
                                       Rc::clone(&self.on)
                                      ))
                          .collect::<HashMap<_, _, DefaultSeaHasher>>();

        let mut k: u8 = 0;

        while pattern.len() > 1 {
            let z = self.get_zero(k);

            let pop = |pattern: &mut HashMap<_, _, _>, x, y| -> Rc<Node> {
                match pattern.remove(&(x, y)) {
                    Some(value) => value,
                    None => Rc::clone(&z)
                }
            };

            let mut next_level = HashMap
                                 ::<(i64, i64), Rc<Node>, DefaultSeaHasher>
                                 ::default();

            while pattern.len() > 0 {
                // can unwrap since pattern.len() > 0
                let ((x, y), _) = &pattern.iter().next().unwrap();
                
                let x = x - x % 2;
                let y = y - y % 2;
                let a = pop(&mut pattern, x, y);
                let b = pop(&mut pattern, x + 1, y);
                let c = pop(&mut pattern, x, y + 1);
                let d = pop(&mut pattern, x + 1, y + 1);
                let r = self.join([a, b, c, d]);

                next_level.insert((x, y), r);
            }
            
            pattern = next_level;
            k += 1;
        }

        assert_eq!(pattern.len(), 1);

        return Rc::clone(&pattern.iter().next().unwrap().1);
    }

    fn join(&mut self, abcd: [Rc<Node>; 4]) -> Rc<Node> {
        let cached = self.join_cache.get(&abcd);

        if let Some(result) = cached {
            return Rc::clone(result);
        }
        
        let [a, b, c, d] = &abcd;

        let hash: u64 = 784753_u64.wrapping_mul(a.k().into())
                                  .wrapping_add(a.hash.wrapping_mul(616207))
                                  .wrapping_add(b.hash.wrapping_mul(990037))
                                  .wrapping_add(c.hash.wrapping_mul(599383))
                                  .wrapping_add(d.hash.wrapping_mul(482263));
        
        let n = a.n() | b.n() | c.n() | d.n();

        let joined = Node::new(a.k() + 1, 
                                         abcd.clone(),
                                         n,
                                         hash);
        
        self.join_cache.put(abcd.clone(), Rc::clone(&joined));

        return joined;
    }

    fn get_zero(&mut self, k: u8) -> Rc<Node> {
        if k == 0 {
            return Rc::clone(&self.off);
        }

        let cached = self.zero_cache.get(&k);
        if let Some(result) = cached {
            return Rc::clone(result);
        }

        let prev = self.get_zero(k - 1);
        let result = self.join([Rc::clone(&prev), 
                                Rc::clone(&prev),
                                Rc::clone(&prev),
                                Rc::clone(&prev)]);

        
        self.zero_cache.insert(k, Rc::clone(&result));

        result
    }

    fn centre(&mut self, m: Rc<Node>) -> Rc<Node> {
        match m.children.clone() {
            None => panic!("centre panic! m should have children"),
            Some([a, b, c, d]) => {
                let zero_to_clone = self.get_zero(m.k() - 1);
                let zero = || Rc::clone(&zero_to_clone);
                
                let ra = self.join([zero(), zero(), zero(), a]);
                let rb = self.join([zero(), zero(), b, zero()]);
                let rc = self.join([zero(), c, zero(), zero()]);
                let rd = self.join([d, zero(), zero(), zero()]);

                self.join([ra, rb, rc, rd])
            }
        }
    }

    fn life_3x3(&self, nodes: [Rc<Node>; 9]) -> Rc<Node> {
        let [a, b, c,
             d, e, f,
             g, h, i] = nodes;
        
        let outer_on_cells: u8 = a.n_u8() + b.n_u8() + c.n_u8()
                               + d.n_u8() + f.n_u8()
                               + g.n_u8() + h.n_u8() + i.n_u8();

        if (outer_on_cells == 2 && e.n()) || outer_on_cells == 3 {
            return Rc::clone(&self.on);
        } else {
            return Rc::clone(&self.off);
        }
    }

    fn life_4x4(&mut self, m: Rc<Node>) -> Rc<Node> {
        if m.k() != 2 {
            panic!("life_4x4: Level of node is wrong!");
        }

        match self.life_4x4_cache.get(&m) {
            Some(x) => Rc::clone(x),
            None => {
                let [a, b, c, d] = m.unwrap_children_cloned();
                let [aa, ab, ac, ad] = a.unwrap_children_cloned();
                let [ba, bb, bc, bd] = b.unwrap_children_cloned();
                let [ca, cb, cc, cd] = c.unwrap_children_cloned();
                let [da, db, dc, dd] = d.unwrap_children_cloned();

                let ab: Rc<Node> = self.life_3x3([&aa, &ab, &ba, 
                                                  &ac, &ad, &bc,
                                                  &ca, &cb, &da].map(Rc::clone));

                let bc: Rc<Node> = self.life_3x3([&ab, &ba, &bb, 
                                                  &ad, &bc, &bd,
                                                  &cb, &da, &db].map(Rc::clone));

                let cb: Rc<Node> = self.life_3x3([&ac, &ad, &bc,
                                                  &ca, &cb, &da,
                                                  &cc, &cd, &dc].map(Rc::clone));

                let da: Rc<Node> = self.life_3x3([&ad, &bc, &bd,
                                                  &cb, &da, &db,
                                                  &cd, &dc, &dd].map(Rc::clone));

                let result = self.join([ab, bc, cb, da]);

                self.life_4x4_cache.insert(m, Rc::clone(&result));
                
                result
            }
        }
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