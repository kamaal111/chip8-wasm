mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Chip8CPU {}

#[wasm_bindgen]
impl Chip8CPU {
    pub fn new() -> Chip8CPU {
        utils::set_panic_hook();
        Chip8CPU {}
    }
}
