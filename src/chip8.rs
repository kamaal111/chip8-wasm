#[path = "./utils.rs"]
mod utils;

#[path = "./chip8_cpu.rs"]
mod chip8_cpu;
use chip8_cpu::Chip8CPU;

use std::collections::HashMap;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct Chip8 {
    cpu: Chip8CPU,
    games: HashMap<String, Vec<u8>>,
}

#[wasm_bindgen]
impl Chip8 {
    pub fn new() -> Chip8 {
        utils::set_panic_hook();

        Chip8 {
            cpu: Chip8CPU::new(),
            games: Self::make_games(),
        }
    }

    /// Load a ROM by name.
    /// Returns void on success and throws a JavaScript on failure.
    pub fn load_rom(&mut self, game_name: String) -> Result<(), js_sys::Error> {
        let game_data = match self.get_game_with_name(game_name) {
            None => return Err(js_sys::Error::new("Invalid game provided").into()),
            Some(game_data) => game_data,
        };

        let mut new_cpu = Chip8CPU::new();
        new_cpu.load_rom(game_data);
        self.cpu = new_cpu;

        Ok(())
    }

    pub fn cycle(&mut self) {
        self.cpu.cycle();
    }

    // pub fn on_key_press(&self, symbol: String, modifier: String) {}

    // pub fn on_key_release(&self, symbol: String, modifier: String) {}

    /// Get games names as a JavaScript Array of strings.
    pub fn get_game_names(&self) -> js_sys::Array {
        self.games
            .iter()
            .map(|game| JsValue::from_str(game.0))
            .collect::<js_sys::Array>()
    }

    /// Get display buffer as a flat JavaScript array.
    pub fn get_display_buffer_array(&self) -> js_sys::Uint8Array {
        self.cpu.display.get_buffer_array()
    }

    /// Get display width.
    pub fn get_display_width(&self) -> u32 {
        self.cpu.display.width
    }

    /// Get display height.
    pub fn get_display_height(&self) -> u32 {
        self.cpu.display.height
    }

    pub fn get_draw_flag(&self) -> bool {
        self.cpu.draw_flag
    }

    pub fn end_cycle(&mut self) {
        self.cpu.draw_flag = false;
    }
}

impl Chip8 {
    fn make_games() -> HashMap<String, Vec<u8>> {
        let mut games = HashMap::new();
        games.insert("PONG".to_string(), include_bytes!("games/PONG").to_vec());
        games.insert("PONG2".to_string(), include_bytes!("games/PONG2").to_vec());
        games.insert("TANK".to_string(), include_bytes!("games/TANK").to_vec());

        games
    }

    fn get_game_with_name(&self, game_name: String) -> Option<&Vec<u8>> {
        self.games.get(&game_name)
    }
}
