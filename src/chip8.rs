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

    pub fn load_rom(&self, game_name: String) -> Result<(), js_sys::Error> {
        let game_data = match self.get_game_with_name(game_name) {
            None => return Err(js_sys::Error::new("Invalid game provided").into()),
            Some(game_data) => game_data,
        };

        self.cpu.load_rom(game_data);

        Ok(())
    }

    pub fn on_key_press(&self, symbol: String, modifier: String) {}

    pub fn on_key_release(&self, symbol: String, modifier: String) {}

    pub fn get_game_names(&self) -> js_sys::Array {
        self.games
            .iter()
            .map(|game| JsValue::from_str(game.0))
            .collect::<js_sys::Array>()
    }

    /// Get buffer as a flat JavaScript array.
    pub fn get_display_buffer(&self) -> js_sys::Uint8Array {
        self.cpu.display.get_buffer()
    }

    pub fn get_display_width(&self) -> u32 {
        self.cpu.display.width
    }

    pub fn get_display_height(&self) -> u32 {
        self.cpu.display.height
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
