use std::collections::{BTreeMap, BTreeSet};
use super::units::{Cell, XCoord, YCoord};
use std::cmp::{min, max};

pub struct Life {
    alive_cells_rows: BTreeMap<YCoord, BTreeSet<XCoord>>
}

impl Life {
    pub fn new(alive_cells: Vec<Cell>) -> Life {
        let mut alive_cells_rows = BTreeMap::<i64, BTreeSet<i64>>::new();

        for c in alive_cells {
            alive_cells_rows
                .entry(c.y)
                .or_default()
                .insert(c.x);
        }

        Life {
            alive_cells_rows
        }
    }

    pub fn alive_cells(&self) -> Vec<Cell> {
        let mut result = vec![];
        
        for (&y, row) in &self.alive_cells_rows {
            for &x in row {
                result.push(Cell{x, y});
            }
        }

        result
    }

    #[inline]
    fn is_cell_alive(&self, x: i64, y: i64) -> bool {
        let row = self.alive_cells_rows.get(&y);

        return match row {
            None => false,
            Some(row) => {row.contains(&x)}
        };
    }

    #[inline]
    fn alive_neighbor_count(&self, x: i64, y: i64) -> u8 {
        let mut count = 0;
        for dx in -1_i64..=1 {
            for dy in -1_i64..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let yy = y + dy;
                let xx = x + dx;
                count += self.is_cell_alive(xx, yy) as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut rows_to_compute = BTreeSet::new();
        let mut min_x : i64 = 0;
        let mut max_x : i64 = 0;

        for (y, row) in &self.alive_cells_rows {
            rows_to_compute.insert(y-1);
            rows_to_compute.insert(*y);
            rows_to_compute.insert(y+1);
            for x in row {
                min_x = min(min_x, *x);
                max_x = max(max_x, *x);
            }
        }

        let mut result = BTreeMap::<YCoord, BTreeSet<XCoord>>::new();

        for y in rows_to_compute {
            for x in min_x-1..=max_x+1 {
                let neighbours = self.alive_neighbor_count(x, y);
                let cell = self.is_cell_alive(x, y);
                
                let next_gen = match (cell, neighbours) {
                    (false, 3) => true,
                    (true, 2..=3) => true,
                    _ => false
                };

                if next_gen {
                    result
                        .entry(y)
                        .or_default()
                        .insert(x);
                }
            }
        }

        self.alive_cells_rows = result  
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
        glider.tick();

        let alive_cells = glider.alive_cells();
        assert_eq!(alive_cells.len(), 5);
        assert!(alive_cells.contains(&Cell{x: 0, y: 1}));
        assert!(alive_cells.contains(&Cell{x: 2, y: 1}));
        assert!(alive_cells.contains(&Cell{x: 1, y: 2}));
        assert!(alive_cells.contains(&Cell{x: 2, y: 2}));
        assert!(alive_cells.contains(&Cell{x: 1, y: 3}));
    }
}