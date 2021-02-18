use std::fs;
use std::thread;
use std::time::Duration;

use crate::chip8::{Chip8, new_chip8};

pub struct Platform {
    chip: Chip8,
    clock: u8,
}

impl Platform {
    pub fn init(&mut self) {
        self.chip.init();
    }

    pub fn open_rom(&mut self, path: &str) {
        let rom_data = fs::read(path).expect("Error opening rom");
        self.chip.load_rom(rom_data.as_slice());
        self.chip.print_memory();
    }

    pub fn handle_input(&mut self, code: &str, is_pressed: bool) {
        if code == "1" {
            self.chip.set_key(0, is_pressed);
        } else if code == "2" {
            self.chip.set_key(1, is_pressed);
        } else if code == "3" {
            self.chip.set_key(2, is_pressed);
        } else if code == "4" {
            self.chip.set_key(3, is_pressed);
        } else if code == "Q" {
            self.chip.set_key(4, is_pressed);
        } else if code == "W" {
            self.chip.set_key(5, is_pressed);
        } else if code == "E" {
            self.chip.set_key(6, is_pressed);
        } else if code == "R" {
            self.chip.set_key(7, is_pressed);
        } else if code == "A" {
            self.chip.set_key(8, is_pressed);
        } else if code == "S" {
            self.chip.set_key(9, is_pressed);
        } else if code == "D" {
            self.chip.set_key(10, is_pressed);
        } else if code == "F" {
            self.chip.set_key(11, is_pressed);
        } else if code == "Y" {
            self.chip.set_key(12, is_pressed);
        } else if code == "X" {
            self.chip.set_key(13, is_pressed);
        } else if code == "C" {
            self.chip.set_key(14, is_pressed);
        } else if code == "V" {
            self.chip.set_key(15, is_pressed);
        } else {
            println!("Error");
        }
    }
}

pub fn new_platform() -> Platform {
    return Platform {
        chip: new_chip8(),
        clock: 60,
    };

}