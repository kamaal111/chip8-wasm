#[path = "./utils.rs"]
mod utils;

#[path = "./traits.rs"]
mod traits;
use traits::FillableVector;

use std::collections::HashMap;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

const DISPLAY_WIDTH: u32 = 64;
const DISPLAY_HEIGHT: u32 = 32;

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
pub struct Chip8Emulator {
    cpu: Chip8CPU,
    games: HashMap<String, Vec<u8>>,
}

#[wasm_bindgen]
impl Chip8Emulator {
    pub fn new() -> Chip8Emulator {
        utils::set_panic_hook();

        Chip8Emulator {
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

impl Chip8Emulator {
    fn make_games() -> HashMap<String, Vec<u8>> {
        let mut games = HashMap::new();
        games.insert("PONG".to_string(), include_bytes!("games/PONG").to_vec());

        games
    }

    fn games_contain_game_name(&self, game_name: String) -> bool {
        self.games.get(&game_name).is_some()
    }
}

struct Chip8CPU {
    /// For the CHIP8 virtual machine, the input comes from a 16-button keyboard
    /// (pretty convenient that the number of keys falls within a nibble). The
    /// machine is also fed with the programs it is supposed to run.
    key_inputs: Vec<u8>,
    /// Display reference.
    display: Display,
    /// CHIP8 has memory that can hold up to 4096 bytes. This includes the
    /// interpreter itself, the fonts (more on this later), and where it loads the
    /// program it is supposed to run (from input).
    memory: Vec<u8>,
    /// The CHIP8 has 16 8-bit registers (usually referred to as Vx where x is the
    /// register number in Cogwood's reference). These are generally used to store
    /// values for operations. The last register, Vf, is mostly used for flags and
    /// should be avoided for use in programs.
    gpio: Vec<u8>,
    /// 8-bit sound timer
    sound_timer: u8,
    /// 8-bit delay timer
    delay_timer: u8,
    /// 16-bit index register
    index_register: u16,
    /// 16-bit program counter
    program_counter: u16,
    /// A stack of at most 16 16-bit values, used for subroutine calls.
    stack_pointer: Vec<u16>,
}

impl Chip8CPU {
    fn new() -> Chip8CPU {
        Chip8CPU {
            key_inputs: Vec::with_filled_capacity(16, 0),
            display: Display::new(),
            memory: Vec::with_filled_capacity(4096, 0),
            gpio: Vec::with_filled_capacity(16, 0),
            sound_timer: 0,
            delay_timer: 0,
            index_register: 0,
            program_counter: 0,
            stack_pointer: Vec::with_capacity(16),
        }
    }
}

/// For output, the machine uses a 64x32 display, and a simple sound buzzer.
/// The display is basically just an array of pixels that are either in the
/// on or off state.
struct Display {
    width: u32,
    height: u32,
    buffer: Vec<u8>,
}

impl Display {
    fn new() -> Display {
        Display {
            width: DISPLAY_WIDTH,
            height: DISPLAY_HEIGHT,
            buffer: Vec::with_filled_capacity((DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize, 0),
        }
    }

    fn get_buffer(&self) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(self.buffer.as_slice())
    }
}
