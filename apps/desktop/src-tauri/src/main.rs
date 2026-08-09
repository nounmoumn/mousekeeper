#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(feature = "tauri-commands")]
fn main() {
    mousekeeper_desktop::run();
}

#[cfg(not(feature = "tauri-commands"))]
fn main() {}
