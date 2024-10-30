mod utils;
mod rle_parser;
mod life;
mod units;

use life::Life;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
struct Universe {
    life: Life,
    output: Vec<i64>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(s: String) -> Universe {
        Universe {
            life: Life::new(rle_parser::read(s)),
            output: vec![]
        }
    }

    pub fn tick(&mut self, log_times: u8) {
        self.life.tick(log_times);
    }

    pub fn req_output(&mut self) {
        self.output.clear();
        let alive_cells = self.life.alive_cells(0, 0, 0, 100, 100);
        for item in alive_cells {
            self.output.push(item.x);
            self.output.push(item.y);
        }
    }

    pub fn output_len(&self) -> usize {
        return self.output.len();
    }

    pub fn output(&self) -> *const i64 { 
        return self.output.as_ptr();
    }
}
