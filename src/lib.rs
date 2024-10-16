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
    output: Vec<u8>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(s: String) -> Universe {
        Universe {
            life: Life::new(rle_parser::read(s)),
            output: vec![]
        }
    }

    pub fn read(&self, x: String) -> *const u8 { 
        return self.output.as_ptr();
    }   
}
