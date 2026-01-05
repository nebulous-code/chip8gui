//! This module defines the binary entry point for the Chip-8 GUI.

/// This function starts the Chip-8 GUI application.
/// Arguments: none.
/// Returns: none.
fn main() {
    // This call runs the GUI loop from the library crate.
    chip8gui::run();
}
