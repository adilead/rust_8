use crate::chip8::{Chip8, new_chip8};
use std::time::Duration;
use std::thread;


pub struct Platform {
    chip: Chip8,
    clock: u8
}

impl Platform {

    pub fn init(&mut self) {
        self.chip.init();
    }

    pub fn execution_loop(&mut self) {
        loop {
            // thread::sleep(Duration::from_millis(1000/60)); //Idk what clock cycle ?
            self.chip.cycle();
        }
    }
}

pub fn new_platform() -> Platform{
    return Platform{
        chip: new_chip8(),
        clock: 60
    }

}