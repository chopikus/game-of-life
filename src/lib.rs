use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
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

        Universe {
            width,
            height,
            cells,
        }
    }


    pub fn render(&self) -> String {
        /* Returning a list of black cells for now. */
        use serde_json::json;
        let mut alive_cells : Vec<(u32, u32)> = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.get_cells_index(x, y);
                if self.cells[idx] == Cell::Alive {
                    alive_cells.push((x, y));
                }
            }
        }
        
        json!(alive_cells).to_string()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let alive_neighbours = self.alive_neighbour_count(x, y);

                let idx = self.get_cells_index(x, y);
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
    
    fn get_cells_index(&self, x: u32, y: u32) -> usize {
        (self.width * y + x) as usize
    } 

    fn alive_neighbour_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;
        for dx in [self.width - 1, 0, 1].into_iter() {
            for dy in [self.height - 1, 0, 1].into_iter() {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let neighbour_x = (x + dx) % self.width;
                let neighbour_y = (y + dy) % self.height;
                let idx = self.get_cells_index(neighbour_x, neighbour_y);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}
