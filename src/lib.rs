mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
struct Universe {
    output: Vec<u8>,  
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        Universe {
            output: vec![]
        }
    }

    pub fn read(&mut self, x: String) -> *const u8 { 
        //log(&x);
        
        let mut my_vec= vec![];
        
        for i in 0..1048576 {
            let val = (i % 255) as u8;
            my_vec.push(val);
        }

        self.output = my_vec;

        return self.output.as_ptr();
    }   
}
