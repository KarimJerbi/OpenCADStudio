#![allow(non_snake_case)]
// On Windows release builds, hide the console window the OS would
// otherwise spawn alongside the GUI. Debug builds keep stdout/stderr
// attached so eprintln! / panics stay visible while developing.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod app;
mod command;
mod entities;
mod io;
mod linetypes;
mod modules;
mod patterns;
mod scene;
mod snap;
mod ui;
mod update_check;

fn main() -> iced::Result {
    app::run()
}
