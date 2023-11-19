#[path = "./traits.rs"]
mod traits;
use traits::FillableVector;

#[path = "./display.rs"]
mod display;
use display::Display;

use wasm_bindgen::prelude::wasm_bindgen;

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// To print in to the browser console.
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub struct Chip8CPU {
    /// For the CHIP8 virtual machine, the input comes from a 16-button keyboard
    /// (pretty convenient that the number of keys falls within a nibble). The
    /// machine is also fed with the programs it is supposed to run.
    key_inputs: Vec<u8>,
    /// Display reference.
    pub display: Display,
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
    /// Whether or not to draw.
    pub draw_flag: bool,
}

impl Chip8CPU {
    pub fn new() -> Chip8CPU {
        Chip8CPU {
            key_inputs: Vec::with_filled_capacity(16, 0),
            display: Display::new(),
            memory: load_fontset(Vec::with_filled_capacity(4096, 0)),
            gpio: Vec::with_filled_capacity(16, 0),
            sound_timer: 0,
            delay_timer: 0,
            index_register: 0,
            program_counter: 0,
            stack_pointer: Vec::with_capacity(16),
            draw_flag: false,
        }
    }

    pub fn load_rom(&mut self, game_data: &Vec<u8>) {
        console_log!("Loaded game with {:?} bytes", game_data.len());
        game_data
            .iter()
            .enumerate()
            .for_each(|(index, binary)| self.memory[index + 0x200] = binary.clone());
    }

    pub fn cycle(&mut self) {
        self.process_opcode();
        self.program_counter += 2;
        self.update_timers();
    }
}

impl Chip8CPU {
    fn get_opcode(&self) -> u16 {
        let counter = self.program_counter as usize;

        ((self.memory[counter] as u16) << 8) | (self.memory[counter + 1] as u16)
    }

    fn process_opcode(&self) {
        let opcode = self.get_opcode();

        // TODO: Process opcode
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1
        }
        if self.sound_timer == 0 {
            // TODO: Play sound! 🔊
        }
    }
}

fn load_fontset(memory: Vec<u8>) -> Vec<u8> {
    let mut memory_clone = memory.clone();
    FONTSET
        .iter()
        .enumerate()
        .for_each(|(index, font)| memory_clone[index] = font.clone());

    memory_clone
}
