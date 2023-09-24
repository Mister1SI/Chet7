//#![windows_subsystem = "windows"]
use chet7::lib::Chet7;

use iced::{Settings, Application};

fn main() {
    Chet7::run(Settings::default()).unwrap();
}
