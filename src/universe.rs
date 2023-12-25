use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}


/* 
 * Cell indices example:
 * width = 4, height = 3
 * cells:
 * [0 1 2 3
 *  4 5 6 7
 *  8 9 10 11]
 */
#[wasm_bindgen]
impl Universe {
    pub fn new(w: usize, h: usize, cells: Vec<Cell>) -> Universe {
        if w == 0 || h == 0 {
            panic!("Universe::new(): w, h must be >0");
        }
        if cells.len() != w*h {
            panic!("Universe::new(): expected vector of size {w}*{h}");
        }
        Universe {
            width: w,
            height: h,
            cells: cells
        }
    }

    pub fn example() -> Universe {
        let width = 64;
        let height = 64;
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
             })
            .collect();

        Universe::new(width, height, cells)
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let alive_neighbours = self.alive_neighbour_count(x, y);

                let idx = self.get_cell_index(x, y);
                let cell = self.cells[idx];

                let next_cell = match (cell, alive_neighbours) {
                    (Cell::Dead, 3) => Cell::Alive,
                    (Cell::Alive, 2..=3) => Cell::Alive,
                    _ => Cell::Dead,
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
    
    pub fn alive_cells_str(&self) -> String {
        use serde_json::json;
        json!(self.alive_cells()).to_string()
    }
}

impl Universe {
    pub fn alive_cells(&self) -> Vec<(usize, usize)> {
        let mut alive_cells : Vec<(usize, usize)> = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.get_cell_index(x, y);
                if self.cells[idx] == Cell::Alive {
                    alive_cells.push((x, y));
                }
            }
        }
        alive_cells
    }

 
    fn get_cell_index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    } 

    fn alive_neighbour_count(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dx in [self.width - 1, 0, 1].into_iter() {
            for dy in [self.height - 1, 0, 1].into_iter() {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let neighbour_x = (x + dx) % self.width;
                let neighbour_y = (y + dy) % self.height;
                let idx = self.get_cell_index(neighbour_x, neighbour_y);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn glider() -> Universe {
        let width = 5;
        let height = 5;
        let cells = vec![Cell::Dead; width*height];
        let mut glider = Universe {
            width,
            height,
            cells
        };
        let alive_cells = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
        for cell in alive_cells {
            let idx = glider.get_cell_index(cell.0, cell.1);
            glider.cells[idx] = Cell::Alive;
        }
        glider
    }

    #[test]
    fn test_start_cells() {
        let glider = glider();
        let alive_cells = glider.alive_cells();
        assert_eq!(alive_cells.len(), 5);
        assert!(alive_cells.contains(&(1, 0)));
        assert!(alive_cells.contains(&(2, 1)));
        assert!(alive_cells.contains(&(0, 2)));
        assert!(alive_cells.contains(&(1, 2)));
        assert!(alive_cells.contains(&(2, 2)));
    }

    #[test]
    fn test_tick_cells() {
        let mut glider = glider();
        glider.tick();
        let alive_cells = glider.alive_cells();
        assert_eq!(alive_cells.len(), 5);
        assert!(alive_cells.contains(&(0, 1)));
        assert!(alive_cells.contains(&(2, 1)));
        assert!(alive_cells.contains(&(1, 2)));
        assert!(alive_cells.contains(&(2, 2)));
        assert!(alive_cells.contains(&(1, 3)));       
    }
}
