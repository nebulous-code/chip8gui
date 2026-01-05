#![crate_name = "chip8gui"]

use std::fs;
use std::path::Path;

use chip8sys::chip8::{Chip8KeyMask, Chip8Sys, DISPLAY_PIXELS};
use minifb::{Key, ScaleMode, Window, WindowOptions};
use rodio::source::{SineWave, Source};

/// This constant defines the color used for pixels that are off.
const PIXEL_COLOR_OFF: u32 = 0x3D521E;
/// This constant defines the color used for pixels that are on.
const PIXEL_COLOR_ON: u32 = 0x80B039;
/// This constant defines the window width in pixels.
pub const WIDTH: usize = 640  * 2;
/// This constant defines the window height in pixels.
pub const HEIGHT: usize = 320 * 2;
/// This constant controls whether FX55 and FX65 increment the index register.
pub const INC_INDEX: bool = true;
/// This constant controls whether some opcodes reset register VF.
pub const VF_RESET: bool = true;
/// This constant controls whether sprites wrap around screen edges when drawn.
pub const WRAP_DRAW: bool = false;
/// This constant controls whether shifts modify VX in place.
pub const MOD_VX_IN_PLACE: bool = false;

/// This function runs the Chip-8 GUI application loop.
/// Arguments: none.
/// Returns: none.
pub fn run() {
    // This creates a new emulator instance.
    let mut game = Chip8Sys::new_chip_8();

    // This selects a ROM file name to load from disk.
    // let rom_name = "1-chip8-logo.ch8";
    // let rom_name = "2-ibm-logo.ch8";
    // let rom_name = "3-corax+.ch8";
    // let rom_name = "4-flags.ch8";
    // let rom_name = "5-quirks.ch8";
    // let rom_name = "7-beep.ch8";
    // This optional memory patch starts the quirks ROM automatically.
    // game.memory[0x1FF] = 1;
    // let rom_name = "6-keypad.ch8";
    let rom_name = "walking_man.ch8";

    // This builds an absolute path to the ROM file based on the crate directory.
    let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../roms")
        .join(rom_name);

    // This reads the ROM file into memory.
    let rom_bytes = fs::read(&file_path).expect("rom file should be readable");
    // This loads the ROM bytes into emulator memory.
    game.load_rom_bytes(&rom_bytes);

    // This sets up audio output for the beep tone.
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    // This sink plays the audio tone on demand.
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());
    // sink.append(SineWave::new(440.0).repeat_infinite());

    // This creates the window for rendering.
    let mut window = Window::new(
        "Chip 8 - Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create the window");

    // This caps the frame rate to reduce CPU usage.
    window.set_target_fps(240);

    // This loop runs until the window closes or the user exits.
    let mut buffer;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // This updates the keypad state from keyboard input.
        check_key_input(&mut game, &window);
        // This converts the framebuffer to a display buffer.
        buffer = display_buffer(&game);
        // This draws the buffer to the window.
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        // This advances the emulator state by one CPU cycle.
        let _ = game.tick(1);
        // This toggles the beep tone based on the sound flag.
        if game.is_sound_playing() {
            sink.append(SineWave::new(440.0).repeat_infinite());
        } else {
            sink.stop();
        }
    }
}

/// This function maps keyboard input to the Chip-8 keypad state.
/// Arguments:
/// - chip8: The emulator instance to update.
/// - window: The window that provides key state data.
/// Returns: none.
fn check_key_input(chip8: &mut Chip8Sys, window: &Window) {
    // This mapping follows the standard Chip-8 keypad layout.
    // This mask stores the pressed keys in a 16-bit value.
    let mut mask: Chip8KeyMask = 0;
    if window.is_key_down(Key::X) {
        mask |= 1u16 << 0;
    }
    if window.is_key_down(Key::Key1) {
        mask |= 1u16 << 1;
    }
    if window.is_key_down(Key::Key2) {
        mask |= 1u16 << 2;
    }
    if window.is_key_down(Key::Key3) {
        mask |= 1u16 << 3;
    }
    if window.is_key_down(Key::Q) {
        mask |= 1u16 << 4;
    }
    if window.is_key_down(Key::W) {
        mask |= 1u16 << 5;
    }
    if window.is_key_down(Key::E) {
        mask |= 1u16 << 6;
    }
    if window.is_key_down(Key::A) {
        mask |= 1u16 << 7;
    }
    if window.is_key_down(Key::S) {
        mask |= 1u16 << 8;
    }
    if window.is_key_down(Key::D) {
        mask |= 1u16 << 9;
    }
    if window.is_key_down(Key::Z) {
        mask |= 1u16 << 0xA;
    }
    if window.is_key_down(Key::C) {
        mask |= 1u16 << 0xB;
    }
    if window.is_key_down(Key::Key4) {
        mask |= 1u16 << 0xC;
    }
    if window.is_key_down(Key::R) {
        mask |= 1u16 << 0xD;
    }
    if window.is_key_down(Key::F) {
        mask |= 1u16 << 0xE;
    }
    if window.is_key_down(Key::V) {
        mask |= 1u16 << 0xF;
    }
    // This updates the emulator keypad using the boundary API.
    chip8.set_keys_mask(mask);
}

/// This function converts the Chip-8 framebuffer into a scaled display buffer.
/// Arguments:
/// - chip8: The emulator instance that owns the framebuffer.
/// Returns: A vector of packed pixel colors for the window.
pub fn display_buffer(chip8: &Chip8Sys) -> Vec<u32> {
    // This uses the fixed Chip-8 pixel count to calculate the scale factor.
    // This uses a square root because the scale is applied to width and height.
    let scaler = ((WIDTH * HEIGHT) as f64 / DISPLAY_PIXELS as f64)
        .sqrt()
        .floor() as usize;
    // println!("scaler: {scaler}");
    // let scaler = 20;

    // This optional call prints a debug view of the framebuffer.
    // self.debug_print_frame_buffer();

    let mut results = Vec::new();
    let mut result: Vec<u32> = Vec::new();
    for (i, pixel) in chip8.framebuffer_packed().iter().enumerate() {
        let mut power_2 = 0b1000_0000;
        for _ in 0..8 {
            if pixel & power_2 == power_2 {
                result.append(&mut vec![PIXEL_COLOR_ON; scaler]);
            } else {
                result.append(&mut vec![PIXEL_COLOR_OFF; scaler]);
            }
            // This shifts the mask to check the next pixel bit.
            power_2 /= 2;
        }
        // This expands the row vertically after each 8 bytes (64 pixels).
        if (i + 1) % 8 == 0 {
            results.append(&mut vec![result; scaler].concat());
            result = Vec::new();
        }
    }
    results
}
