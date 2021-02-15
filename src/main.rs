mod chip8;
mod platform;

fn main() {
    println!("Hello, world!");

    let c8 = chip8::new_chip8();
    c8.chip8_says_hello();

    let platform = platform::new_platform();

    let x = 2u32;
    let y = 3;

    print!("{}", x & y);
}
