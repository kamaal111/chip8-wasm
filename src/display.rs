#[path = "./traits.rs"]
mod traits;
use traits::FillableVector;

const DISPLAY_WIDTH: u32 = 64;
const DISPLAY_HEIGHT: u32 = 32;

/// For output, the machine uses a 64x32 display, and a simple sound buzzer.
/// The display is basically just an array of pixels that are either in the
/// on or off state.
pub struct Display {
    pub width: u32,
    pub height: u32,
    buffer: Vec<u8>,
}

impl Display {
    pub fn new() -> Display {
        Display {
            width: DISPLAY_WIDTH,
            height: DISPLAY_HEIGHT,
            buffer: Vec::with_filled_capacity((DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize, 0),
        }
    }

    pub fn get_buffer(&self) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(self.buffer.as_slice())
    }
}
