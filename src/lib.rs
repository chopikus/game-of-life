use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[allow(unused_variables)]
fn main() {
    extern crate wasm_bindgen;

    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern {
        fn alert(s: &str);
    }
    
    #[wasm_bindgen]
    pub fn greet() {
        utils::set_panic_hook();
        alert("Hello from Ihor");
        panic!("wtf");
    }
}

mod utils;
