#[path = "./traits.rs"]
mod traits;

use traits::FillableVector;

#[path = "./display.rs"]
mod display;
use display::Display;

use rand::Rng;

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
    stack: Vec<u16>,
    /// Current stack pointer index.
    stack_pointer: u8,
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
            stack: Vec::with_filled_capacity(16, 0),
            stack_pointer: 0,
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
        self.update_timers();
    }
}

impl Chip8CPU {
    fn get_opcode(&self) -> u16 {
        let counter = self.program_counter as usize;

        ((self.memory[counter] as u16) << 8) | (self.memory[counter + 1] as u16)
    }

    fn process_opcode(&mut self) {
        let opcode = self.get_opcode();

        match opcode & 0xF000 {
            0x0000 => match opcode & 0x000F {
                0x0000 => {
                    // 00E0: Clears the screen.
                    self.display = Display::new();
                    self.draw_flag = true;
                    self.program_counter += 2;
                    return;
                }
                0x000E => {
                    // 00EE: Returns from a subroutine.
                    let stack_pointer = self.stack_pointer - 1;
                    self.stack_pointer = stack_pointer;
                    self.program_counter = self.stack[stack_pointer as usize] + 2;
                    return;
                }
                _ => {
                    panic!("Unknown opcode in default 0x0000 {:X}", opcode);
                }
            },
            0x1000 => {
                // 1NNN: Jumps to address NNN.
                self.program_counter += 2;
                return;
            }
            0x2000 => {
                // 2NNN: Calls subroutine at NNN.
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;
                self.program_counter = opcode & 0x0FFF;
                return;
            }
            0x3000 => {
                // 3XNN: Skips the next instruction if VX equals NN.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.program_counter += if self.gpio[x] == nn { 4 } else { 2 };
                return;
            }
            0x4000 => {
                // 4XNN: Skips the next instruction if VX doesn't equal NN.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.program_counter += if self.gpio[x] != nn { 4 } else { 2 };
                return;
            }
            0x5000 => {
                // 5XY0: Skips the next instruction if VX equals VY.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x0F00) >> 4) as usize;
                self.program_counter += if self.gpio[x] == self.gpio[y] { 4 } else { 2 };
                return;
            }
            0x6000 => {
                // 6XNN: Sets VX to NN.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.gpio[x] = nn;
                self.program_counter += 2;
                return;
            }
            0x7000 => {
                // 7XNN: Adds NN to VX.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                (self.gpio[x], _) = self.gpio[x].overflowing_add(nn);
                self.program_counter += 2;
                return;
            }
            0x8000 => match opcode & 0x000F {
                0x0000 => {
                    // 8XY0: Sets VX to the value of VY.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x0F00) >> 4) as usize;
                    self.gpio[x] = self.gpio[y];
                    self.program_counter += 2;
                    return;
                }
                0x0001 => {
                    // 8XY1: Sets VX to VX or VY.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x0F00) >> 4) as usize;
                    self.gpio[x] |= self.gpio[y];
                    self.program_counter += 2;
                    return;
                }
                0x0002 => {
                    // 8XY2: Sets VX to VX and VY.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x0F00) >> 4) as usize;
                    self.gpio[x] &= self.gpio[y];
                    self.program_counter += 2;
                    return;
                }
                0x0003 => {
                    // 8XY3: Sets VX to VX xor VY.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x0F00) >> 4) as usize;
                    self.gpio[x] ^= self.gpio[y];
                    self.program_counter += 2;
                    return;
                }
                0x0004 => {
                    // 8XY4: Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x0F00) >> 4) as usize;
                    self.gpio[0xF] = if self.gpio[y] > (0xFF - self.gpio[x]) {
                        1
                    } else {
                        0
                    };
                    (self.gpio[x], _) = self.gpio[x].overflowing_add(self.gpio[y]);
                    self.program_counter += 2;
                    return;
                }
                0x0005 => {
                    // 8XY5: VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x0F00) >> 4) as usize;
                    self.gpio[0xF] = if self.gpio[y] > self.gpio[x] { 0 } else { 1 };
                    (self.gpio[x], _) = self.gpio[x].overflowing_sub(self.gpio[y]);
                    self.program_counter += 2;
                    return;
                }
                0x0006 => {
                    // 8XY6: Shifts VX right by one. VF is set to the value of the least significant bit of VX before the shift.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.gpio[0xF] = self.gpio[x] & 0x1;
                    self.gpio[x] >>= 1;
                    self.program_counter += 2;
                    return;
                }
                0x0007 => {
                    // 8XY7: Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x0F00) >> 4) as usize;
                    self.gpio[0xF] = if self.gpio[x] > self.gpio[y] { 0 } else { 1 };
                    (self.gpio[x], _) = self.gpio[y].overflowing_sub(self.gpio[x]);
                    self.program_counter += 2;
                    return;
                }
                // 61584
                0x000E => {
                    // 8XYE: Shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.gpio[0xF] = self.gpio[x] >> 7;
                    self.gpio[x] <<= 1;
                    self.program_counter += 2;
                    return;
                }
                _ => {
                    panic!("Unknown opcode in default 0x8000 {:X}", opcode);
                }
            },
            0x9000 => {
                // 9XY0: Skips the next instruction if VX doesn't equal VY.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x0F00) >> 4) as usize;
                self.program_counter += if self.gpio[x] != self.gpio[y] { 4 } else { 2 };
                return;
            }
            0xA000 => {
                // ANNN: Sets I to the address NNN.
                self.index_register = opcode & 0x0FFF;
                self.program_counter += 2;
                return;
            }
            0xB000 => {
                // BNNN: Jumps to the address NNN plus V0.
                self.program_counter = (opcode & 0x0FFF) + (self.gpio[0] as u16);
                return;
            }
            0xC000 => {
                // CXNN: Sets VX to the result of a bitwise and operation on a random number and NN.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                let mut rng = rand::thread_rng();
                self.gpio[x] = nn & (rng.gen::<u8>() % 0xFF);
                return;
            }
            0xD000 => {
                // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value doesn’t change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that doesn’t happen
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x0F00) >> 4) as usize;
                let height = opcode & 0x000F;
                self.gpio[0xF] = 0;
                (0..height).for_each(|y_line| {
                    let pixel = self.memory[(self.index_register + y_line) as usize];
                    (0..8).for_each(|x_line| {
                        if (pixel & (0x80 & x_line)) == 0 {
                            return;
                        }

                        let display_buffer_location =
                            (x + (x_line as usize) + (y + (y_line as usize))) * 64;
                        let buffer_size = (self.display.width * self.display.height) as usize;
                        if display_buffer_location >= buffer_size {
                            return;
                        }

                        let buffer_item = self.display.get_buffer_item(display_buffer_location);
                        if buffer_item == 1 {
                            self.gpio[0xF] = 1;
                        }
                        self.display
                            .set_buffer_item(display_buffer_location, buffer_item ^ 1);
                    })
                });
                self.draw_flag = false;
                self.program_counter += 2;
                return;
            }
            0xE000 => match opcode & 0x000F {
                0x000E => {
                    // EX9E: Skips the next instruction if the key stored in VX is pressed.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.program_counter += if self.key_inputs[self.gpio[x] as usize] != 0 {
                        4
                    } else {
                        2
                    };
                    return;
                }
                0x0001 => {
                    // EXA1: Skips the next instruction if the key stored in VX isn't pressed.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.program_counter += if self.key_inputs[self.gpio[x] as usize] == 0 {
                        4
                    } else {
                        2
                    };
                    return;
                }
                _ => {
                    panic!("Unknown opcode in default 0xE000 {:X}", opcode);
                }
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => {
                    // FX07: Sets VX to the value of the delay timer.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.gpio[x] = self.delay_timer;
                    self.program_counter += 2;
                    return;
                }
                0x000A => {
                    // FX0A: A key press is awaited, and then stored in VX.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let mut key_pressed = false;
                    (0..self.key_inputs.len()).for_each(|index| {
                        let key = self.key_inputs[index];
                        if key == 0 {
                            return;
                        }

                        self.gpio[x] = index as u8;
                        key_pressed = true;
                    });

                    if !key_pressed {
                        return;
                    }

                    self.program_counter += 2;
                    return;
                }
                0x0015 => {
                    // FX15: Sets the delay timer to VX.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.delay_timer = self.gpio[x];
                    self.program_counter += 2;
                    return;
                }
                0x0018 => {
                    // FX18: Sets the sound timer to VX.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.sound_timer = self.gpio[x];
                    self.program_counter += 2;
                    return;
                }
                0x001E => {
                    // FX1E: Adds VX to I.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    // VF is set to 1 when range overflow (I+VX>0xFFF), and 0 when there isn't.
                    self.gpio[0xF] = if (self.index_register + (self.gpio[x] as u16)) > 0xFFF {
                        1
                    } else {
                        0
                    };
                    self.index_register = self.gpio[x] as u16;
                    self.program_counter += 2;
                    return;
                }
                0x0029 => {
                    // FX29: Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.index_register = (self.gpio[x] as u16) * 0x5;
                    self.program_counter += 2;
                    return;
                }
                0x0033 => {
                    // FX33: Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let memory_index = self.index_register as usize;
                    let x_register = self.gpio[x];
                    self.memory[memory_index] = x_register;
                    self.memory[memory_index + 1] = (x_register / 10) % 10;
                    self.memory[memory_index + 2] = (x_register / 100) % 10;
                    self.program_counter += 2;
                    return;
                }
                0x0055 => {
                    // FX55: Stores V0 to VX (including VX) in memory starting at address I.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    (0..(x + 1)).for_each(|index| {
                        let memory_index = (self.index_register as usize) + index;
                        self.memory[memory_index] = self.gpio[index];
                    });
                    self.index_register += ((opcode & 0x0F00) >> 8) + 1;
                    self.program_counter += 2;
                    return;
                }
                0x0065 => {
                    // FX65: Fills V0 to VX (including VX) with values from memory starting at address I.
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    (0..(x + 1)).for_each(|index| {
                        let memory_index = (self.index_register as usize) + index;
                        self.memory[index] = self.gpio[memory_index];
                    });
                    self.index_register += ((opcode & 0x0F00) >> 8) + 1;
                    self.program_counter += 2;
                    return;
                }
                _ => {
                    panic!("Unknown opcode in default 0xF000 {:X}", opcode);
                }
            },
            _ => {
                panic!("Unknown opcode in all default {:X}", opcode);
            }
        }
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
