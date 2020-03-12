use super::ScreenCharacter;
use crate::{
    colors::{Color16Bit, TextModeColor},
    vga::{Vga, VideoMode, VGA},
};
use spinning_top::SpinlockGuard;

const WIDTH: usize = 40;
const HEIGHT: usize = 50;
const SCREEN_SIZE: usize = WIDTH * HEIGHT;

static BLANK_CHARACTER: ScreenCharacter = ScreenCharacter {
    character: b' ',
    color: TextModeColor::new(Color16Bit::Yellow, Color16Bit::Black),
};

/// A basic interface for interacting with vga text mode 40x50
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use vga::Text40x50;
///
/// let text_mode = Text40x50::new();
///
/// text_mode.set_mode();
/// text_mode.clear_screen();
/// ```
#[derive(Default)]
pub struct Text40x50;

impl Text40x50 {
    /// Creates a new `Text40x50`.
    pub fn new() -> Text40x50 {
        Text40x50 {}
    }

    /// Clears the screen by setting all cells to `b' '` with
    /// a background color of `Color16Bit::Black` and a foreground
    /// color of `Color16Bit::Yellow`.
    pub fn clear_screen(&self) {
        let (_vga, frame_buffer) = self.get_frame_buffer();
        for i in 0..SCREEN_SIZE {
            unsafe {
                frame_buffer.add(i).write_volatile(BLANK_CHARACTER);
            }
        }
    }

    /// Prints the given `character` and `color` at `(x, y)`.
    ///
    /// Panics if `x >= 40` or `y >= 50`.
    pub fn write_character(&self, x: usize, y: usize, character: u8, color: TextModeColor) {
        assert!(x < WIDTH, "x >= {}", WIDTH);
        assert!(y < HEIGHT, "y >= {}", HEIGHT);
        let (_vga, frame_buffer) = self.get_frame_buffer();
        let offset = WIDTH * y + x;
        unsafe {
            frame_buffer
                .add(offset)
                .write_volatile(ScreenCharacter { character, color });
        }
    }

    /// Sets the graphics device to `VideoMode::Mode40x50`.
    pub fn set_mode(&self) {
        VGA.lock().set_video_mode(VideoMode::Mode40x50);
    }

    /// Returns the start of the `FrameBuffer` as `*mut ScreenCharacter`
    /// as well as a lock to the vga driver. This ensures the vga
    /// driver stays locked while the frame buffer is in use.
    fn get_frame_buffer(&self) -> (SpinlockGuard<Vga>, *mut ScreenCharacter) {
        let mut vga = VGA.lock();
        let frame_buffer = vga.get_frame_buffer();
        (vga, u32::from(frame_buffer) as *mut ScreenCharacter)
    }
}