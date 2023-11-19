#[path = "./utils.rs"]
mod utils;

#[path = "./chip8_cpu.rs"]
mod chip8_cpu;
use chip8_cpu::Chip8CPU;

use std::collections::HashMap;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// To print in to the browser console.
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

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
        if !self.games_contain_game_name(game_name) {
            return Err(js_sys::Error::new("Invalid game provided").into());
        }

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

        games
    }

    fn games_contain_game_name(&self, game_name: String) -> bool {
        self.games.get(&game_name).is_some()
    }
}
