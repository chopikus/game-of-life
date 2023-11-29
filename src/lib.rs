use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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
            .map(|id| {
                let (x, y) = ((id as u32) / width, (id as u32) % width);
                
                let result = match (x, y) {
                    (10, 10) | (9..=11, 11) | (10, 12) => Cell::Alive,
                    _ => Cell::Dead,
                };

                if result == Cell::Alive {
                    log(&("Alive cell! ".to_string() + &x.to_string() + " " + &y.to_string()));    
                }

                result
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    /* Note: does not do conversions of x,y to usize prior to calculation. */
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

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let alive_neighbours = self.alive_neighbour_count(x, y);

                let idx = self.get_cells_index(x, y);
                let cell = self.cells[idx];

                let next_cell = match (cell, alive_neighbours) {
                    (Cell::Dead, 3) => Cell::Alive,
                    (Cell::Dead, _) => Cell::Dead,
                    (Cell::Alive, 2..=3) => Cell::Alive,
                    (Cell::Alive, _) => Cell::Dead,
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
}

use std::fmt;
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[allow(unused_variables)]
fn main() {
    extern crate wasm_bindgen;
}

mod utils;
