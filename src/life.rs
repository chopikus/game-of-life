use std::collections::{BTreeMap, HashMap};
use std::hash::BuildHasherDefault;
use std::num::NonZeroUsize;
use std::rc::Rc;

use lru::LruCache;
use nohash_hasher::NoHashHasher;
use seahash::SeaHasher;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::units::NodeHashType;

use super::units::{Cell, XCoord, YCoord, Node};

//#[allow(dead_code)]
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
   life_4x4_cache: Life4x4CacheType,
   successor_cache: SuccessorCacheType
}

type DefaultSeaHasher = BuildHasherDefault<SeaHasher>;
type JoinCacheType = LruCache<[Rc<Node>; 4], Rc<Node>, DefaultSeaHasher>; 

type ZeroCacheType = BTreeMap<u8, Rc<Node>>;
pub type NodeHasher = BuildHasherDefault<NoHashHasher<NodeHashType>>;
type Life4x4CacheType = HashMap<Rc<Node>, Rc<Node>, NodeHasher>;

type SuccessorCacheType = LruCache<(Rc<Node>, u8), Rc<Node>, DefaultSeaHasher>;

//#[allow(dead_code)]
impl Life {
    fn life() -> Life {
       let join_cache_cap = NonZeroUsize::new(6000000).unwrap();
       let join_cache_hasher = BuildHasherDefault::<SeaHasher>::default();
       let join_cache: JoinCacheType = LruCache::with_hasher(join_cache_cap, join_cache_hasher);

       let zero_cache: ZeroCacheType = BTreeMap::new();
       let life_4x4_cache: Life4x4CacheType = HashMap::default();

       let successor_cache_cap = NonZeroUsize::new(6000000).unwrap();
       let successor_cache_hasher = BuildHasherDefault::<SeaHasher>::default();
       let successor_cache: SuccessorCacheType = LruCache::with_hasher(successor_cache_cap, successor_cache_hasher);

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
           life_4x4_cache,
           successor_cache
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
                          .collect::<BTreeMap<_, _>>();
        let mut k: u8 = 0;

        while pattern.len() > 1 {
            let z = self.get_zero(k);

            let pop = |pattern: &mut BTreeMap<_, _>, x, y| -> Rc<Node> {
                match pattern.remove(&(x, y)) {
                    Some(value) => value,
                    None => Rc::clone(&z)
                }
            };

            let mut next_level = BTreeMap
                                 ::<(i64, i64), Rc<Node>>
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

                next_level.insert((x / 2, y / 2), r);
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

                let o_ab: Rc<Node> = self.life_3x3([&aa, &ab, &ba, 
                                                    &ac, &ad, &bc,
                                                    &ca, &cb, &da].map(Rc::clone));
            

                let o_bc: Rc<Node> = self.life_3x3([&ab, &ba, &bb, 
                                                    &ad, &bc, &bd,
                                                    &cb, &da, &db].map(Rc::clone));


                let o_cb: Rc<Node> = self.life_3x3([&ac, &ad, &bc,
                                                    &ca, &cb, &da,
                                                    &cc, &cd, &dc].map(Rc::clone));

                let o_da: Rc<Node> = self.life_3x3([&ad, &bc, &bd,
                                                    &cb, &da, &db,
                                                    &cd, &dc, &dd].map(Rc::clone));

                let result = self.join([o_ab, o_bc, o_cb, o_da]);

                self.life_4x4_cache.insert(m, Rc::clone(&result));
                
                result
            }
        }
    }

    #[allow(unused_variables)]
    fn c1_c9(&mut self, m: Rc<Node>, j: u8) -> [Rc<Node>; 9] {

        let [a, b, c, d] = m.unwrap_children_cloned();
        
        let [aa, ab, ac, ad] = a.unwrap_children_cloned();
        let [ba, bb, bc, bd] = b.unwrap_children_cloned();
        let [ca, cb, cc, cd] = c.unwrap_children_cloned();
        let [da, db, dc, dd] = d.unwrap_children_cloned();

        let c1 = self.successor(a, j);

        let c2_join = self.join([&ab, &ba, &ad, &bc].map(Rc::clone));
        let c2      = self.successor(c2_join, j);

        let c3 = self.successor(b, j);

        let c4_join = self.join([&ac, &ad, &ca, &cb].map(Rc::clone));
        let c4      = self.successor(c4_join, j);

        let c5_join = self.join([&ad, &bc, &cb, &da].map(Rc::clone));
        let c5      = self.successor(c5_join, j);

        let c6_join = self.join([&bc, &bd, &da, &db].map(Rc::clone));
        let c6      = self.successor(c6_join, j);

        let c7 = self.successor(c, j);

        let c8_join = self.join([&cb, &da, &cd, &dc].map(Rc::clone));
        let c8      = self.successor(c8_join, j);

        let c9 = self.successor(d, j);

        return [c1, c2, c3, c4, c5, c6, c7, c8, c9];
    }

    fn successor(&mut self, m: Rc<Node>, mut j: u8) -> Rc<Node> {
        if m.children.is_none() {
            panic!("successor: m doesn't have children!");
        }

        if m.k() == 1 {
            return m;
        }

        if m.k() == 2 {
            let result = self.life_4x4(m);
            return result;
        }

        if j > m.k() - 2 {
            j = m.k() - 2;
        }

        let key = (Rc::clone(&m), j);
        let cache_result = self.successor_cache.get(&key);

        if let Some(x) = cache_result {
            return Rc::clone(&x);
        }

        let [c1, c2, c3,
             c4, c5, c6,
             c7, c8, c9] = self.c1_c9(Rc::clone(&m), j);

        if j < m.k() - 2 {
            let a = self.join([c1.d(), c2.c(), c4.b(), c5.a()]);
            let b = self.join([c2.d(), c3.c(), c5.b(), c6.a()]);
            let c = self.join([c4.d(), c5.c(), c7.b(), c8.a()]);
            let d = self.join([c5.d(), c6.c(), c8.b(), c9.a()]);

            let r = self.join([a, b, c, d]);
            self.successor_cache.put(key, Rc::clone(&r));

            return r;
        } else {
            let a = self.join([&c1, &c2, &c4, &c5].map(Rc::clone));
            let b = self.join([&c2, &c3, &c5, &c6].map(Rc::clone));
            let c = self.join([&c4, &c5, &c7, &c8].map(Rc::clone));
            let d = self.join([&c5, &c6, &c8, &c9].map(Rc::clone));

            let a = self.successor(a, j);
            let b = self.successor(b, j);
            let c = self.successor(c, j);
            let d = self.successor(d, j);

            let r = self.join([a, b, c, d]);
            self.successor_cache.put(key, Rc::clone(&r));

            return r;
        }
    }

    pub fn tick(&mut self, j: u8) {
        let root = Rc::clone(&self.root);
        let suc = self.successor(root, j);
        self.root = self.centre(suc);
    }

    fn append_alive_cells(&self, node: Rc<Node>, output: &mut Vec<Cell>,
                          level: u8,
                          x: i64, y: i64,
                          bounds: [i64; 4]) {
        if !node.n() {
            return;
        }

        let size: i64 = 1 << node.k();
        let [min_x, min_y, max_x, max_y] = bounds;

        if x + size <= min_x || x > max_x {
            return;
        }
        
        if y + size <= min_y || y > max_y {
            return;
        }

        if node.k() == level {
            output.push(Cell{x: x - self.shift_x, y: y - self.shift_y});
            return;
        }

        let offset: i64 = size >> 1;
        let [a, b, c, d] = node.unwrap_children_cloned();

        self.append_alive_cells(a, output, level, x, y, bounds);
        self.append_alive_cells(b, output, level, x + offset, y, bounds);
        self.append_alive_cells(c, output, level, x, y + offset, bounds);
        self.append_alive_cells(d, output, level, x + offset, y + offset, bounds);
    }

    pub fn alive_cells(&self, level: u8, 
                    mut min_x: i64, mut min_y: i64, 
                    mut max_x: i64, mut max_y: i64) -> Vec<Cell> {
        
        min_x += self.shift_x;
        min_y += self.shift_y;

        max_x += self.shift_x;
        max_y += self.shift_y;

        let mut output = Vec::<Cell>::new();
        self.append_alive_cells(Rc::clone(&self.root), &mut output,
                                level, 
                                0, 0, [min_x, min_y, max_x, max_y]);

        return output;
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
        let alive_cells = glider.alive_cells(0, 0, 0, 5, 5);
        assert_eq!(alive_cells.len(), 5);
        assert!(alive_cells.contains(&Cell{x: 1, y: 0}));
        assert!(alive_cells.contains(&Cell{x: 2, y: 1}));
        assert!(alive_cells.contains(&Cell{x: 0, y: 2}));
        assert!(alive_cells.contains(&Cell{x: 1, y: 2}));
        assert!(alive_cells.contains(&Cell{x: 2, y: 2}));
    }

    #[test]
    fn start_cells_bounds() {
        let glider = glider();
        let alive_cells = glider.alive_cells(0, 1, 1, 2, 2);
        assert_eq!(alive_cells.len(), 3);
        assert!(alive_cells.contains(&Cell{x: 2, y: 1}));
        assert!(alive_cells.contains(&Cell{x: 1, y: 2}));
        assert!(alive_cells.contains(&Cell{x: 2, y: 2}));
    }

    #[test]
    fn glider_1_step() {
        let mut glider = glider();
        glider.tick(0);

        let alive_cells = glider.alive_cells(0, 0, 0, 100, 100);
        assert_eq!(alive_cells.len(), 5);
        assert!(alive_cells.contains(&Cell{x: 0, y: 1}));
        assert!(alive_cells.contains(&Cell{x: 2, y: 1}));
        assert!(alive_cells.contains(&Cell{x: 1, y: 2}));
        assert!(alive_cells.contains(&Cell{x: 2, y: 2}));
        assert!(alive_cells.contains(&Cell{x: 1, y: 3}));
    }

    #[test]
    fn glider_2_steps() {
        let mut glider = glider();
        glider.tick(1);

        let alive_cells = glider.alive_cells(0, 0, 0, 100, 100);
        assert_eq!(alive_cells.len(), 5);
        assert!(alive_cells.contains(&Cell{x: 2, y: 1}));
        assert!(alive_cells.contains(&Cell{x: 0, y: 2}));
        assert!(alive_cells.contains(&Cell{x: 2, y: 2}));
        assert!(alive_cells.contains(&Cell{x: 1, y: 3}));
        assert!(alive_cells.contains(&Cell{x: 2, y: 3}));
    }

    #[test]
    fn glider_3_steps() {
        let mut glider = glider();
        glider.tick(1);
        glider.tick(0);

        let alive_cells = glider.alive_cells(0, 0, 0, 100, 100);
        assert_eq!(alive_cells.len(), 5);
        assert!(alive_cells.contains(&Cell{x: 1, y: 1}));
        assert!(alive_cells.contains(&Cell{x: 2, y: 2}));
        assert!(alive_cells.contains(&Cell{x: 3, y: 2}));
        assert!(alive_cells.contains(&Cell{x: 1, y: 3}));
        assert!(alive_cells.contains(&Cell{x: 2, y: 3}));
    }

    #[test]
    fn glider_1048576_steps() {
        let mut glider = glider();
        glider.tick(20);

        let alive_cells = glider.alive_cells(0, 0, 0, 1000000, 1000000);
        assert_eq!(alive_cells.len(), 5);
        assert!(alive_cells.contains(&Cell{x: 262145, y: 262144}));
        assert!(alive_cells.contains(&Cell{x: 262146, y: 262145}));
        assert!(alive_cells.contains(&Cell{x: 262144, y: 262146}));
        assert!(alive_cells.contains(&Cell{x: 262145, y: 262146}));
        assert!(alive_cells.contains(&Cell{x: 262146, y: 262146}));
    }

    #[test]
    fn test_life_3x3_off() {
        let life = Life::life();
        let on = Rc::clone(&life.on);
        let off = Rc::clone(&life.off);

        let state = [&on, &on, &on,
                     &on, &on, &on,
                     &on, &on, &on].map(Rc::clone);

        assert_eq!(life.life_3x3(state), off);
    }

    #[test]
    fn test_life_3x3_on() {
        let life = Life::life();
        let on = Rc::clone(&life.on);
        let off = Rc::clone(&life.off);

        let state = [&off, &on, &off,
                     &on, &off, &off,
                     &on, &off, &off].map(Rc::clone);

        assert_eq!(life.life_3x3(state), on);
    }

    fn gen_node_4x4(life: &mut Life, state: [Rc<Node>; 16]) -> Rc<Node> {
        let a = life.join([&state[0], &state[1],
                           &state[4], &state[5]].map(Rc::clone));

        let b = life.join([&state[2], &state[3],
                           &state[6], &state[7]].map(Rc::clone));

        let c = life.join([&state[8], &state[9],
                           &state[12], &state[13]].map(Rc::clone));

        let d = life.join([&state[10], &state[11],
                           &state[14], &state[15]].map(Rc::clone));

        return life.join([a, b, c, d]);
    }

    #[test]
    fn test_life_4x4_off() {
        let mut life = Life::life();
        let on = Rc::clone(&life.on);
        let off = Rc::clone(&life.off);

        let state = [&off, &on, &on, &on,
                     &on, &off, &off, &on,
                     &on, &off, &off, &on,
                     &off, &on, &on, &off
                     ].map(Rc::clone);
        
        let input = gen_node_4x4(&mut life, state);

        let children = [&off, &off,
                        &off, &off].map(Rc::clone);

        let output = life.join(children);
        assert_eq!(life.life_4x4(input), output);
    }

    #[test]
    fn test_life_4x4_on() {
        let mut life = Life::life();
        let on = Rc::clone(&life.on);
        let off = Rc::clone(&life.off);

        let state = [&off, &off, &off, &off,
                     &off, &on, &on, &off,
                     &off, &on, &on, &off,
                     &off, &off, &off, &off
                     ].map(Rc::clone);
        
        let input = gen_node_4x4(&mut life, state);

        let children = [&on, &on,
                        &on, &on].map(Rc::clone);

        let output = life.join(children);
        assert_eq!(life.life_4x4(input), output);
    }
}