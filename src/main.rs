#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]
mod native;
fn main() {
    native::run();
}
