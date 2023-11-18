#[path = "./utils.rs"]
mod utils;

#[path = "./traits.rs"]
mod traits;
use traits::FillableVector;

use js_sys::Uint8Array;
use wasm_bindgen::prelude::wasm_bindgen;

const DISPLAY_WIDTH: u32 = 64;
const DISPLAY_HEIGHT: u32 = 32;

#[wasm_bindgen]
pub struct Chip8CPU {
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
#[wasm_bindgen]
impl Chip8CPU {
    /// Chip8 initializer
    pub fn new() -> Chip8CPU {
        utils::set_panic_hook();

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

    pub fn get_display_buffer(&self) -> Uint8Array {
        self.display.get_buffer()
    }

    pub fn get_display_width(&self) -> u32 {
        self.display.width
    }

    pub fn get_display_height(&self) -> u32 {
        self.display.height
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

    /// Get buffer as a flat JavaScript array.
    fn get_buffer(&self) -> Uint8Array {
        Uint8Array::from(self.buffer.as_slice())
    }
}
