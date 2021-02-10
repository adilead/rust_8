use std::vec::Vec;

const START_ADDR: u16 = 0x200;

pub struct Chip8 {
    v: [u8; 16], //Registers for the CPU
    mem: [u8; 4096], //4KB of memory
    opcode: u16,//current opcode
    i: u16,//stores memory address for use in operations
    pc: u16, //Prpgram counter
    stack: [u8; 16], //Stack
    sp : u8, //Stack pointer
    delay: u8, //delay timer; TODO Find a clockspeed
    sound: u8, //sound timer; when the it's 0, a buzz shall be emitted
    gfx: [u8; 32 * 64] //Monochrome display memory
}

impl Chip8 {
    pub fn init(&mut self){

        self.pc = START_ADDR;
        //Storing fonts in the memory
        let fonts: [u8; 80]= [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        for i in 0..80 {
            self.mem[0x50 + i] = fonts[i];
        }


    }

    pub fn load_rom(data: &[u8]){

    }

    pub fn cycle(&mut self) {
        //Fetch
        self.opcode = ((self.mem[self.pc as usize] << 8) as u16 | (self.mem[(self.pc+1) as usize]) as u16);
        //Increment PC
        self.pc += 2;
        //Execute
        self.exec();
        //Delay
        if self.delay > 0 {
            self.delay -= 1;
        }

        //Sound
        if self.sound > 0 {
            self.sound -= 1;
        }
    }


    fn exec(&mut self) {
        match self.opcode {
            0 => println!("bla"),
            _ => print!("bs")
        }
    }

    fn op_00e0(&mut self) { //CLS. Clear the display

    }
    fn op_00ee(&mut self) { //RET


    }
    fn op_1nnn(&mut self) { //JP addr

    }
    fn op_2nnn(&mut self) { //CALL addr

    }
    fn op_3xkk(&mut self) { //SE Vx, byte. skip next instruction if Vx = kk.

    }
    fn op_4xkk(&mut self) { //SNE Vx, byte. Skip next instruction if Vx != kk.

    }
    fn op_5xy0(&mut self) { //SE Vx, Vy. Skip next instruction if Vx = Vy.

    }
    fn op_6xkk(&mut self) { //LD Vx, byte. Set Vx = kk.

    }
    fn op_7xkk(&mut self) { //ADD Vx, byte

    }
    fn op_8xy0(&mut self) { //LD Vx, Vy. Set Vx = Vy

    }
    fn op_8xy1(&mut self) { //OR Vx, Vy. Vx OR Vy.


    }
    fn op_8xy2(&mut self) { //AND Vx, Vy. Set Vx = Vx AND Vy

    }
    fn op_8xy3(&mut self) { //XOR Vx, Vy. Set Vx = Vx XOR Vy.

    }
    fn op_8xy4(&mut self) { //ADD Vx, Vy. Set Vx = Vx + Vy, set VF = carry.

    }
    fn op_8xy5(&mut self) {//SUB Vx, Vy.  Set Vx = Vx - Vy, set VF = NOT borrow.

    }
    fn op_8xy6(&mut self) {//SHR Vx {, Vy}. et Vx = Vx SHR 1.

    }
    fn op_8xy7(&mut self) {//SUBN Vx, Vy. Set Vx = Vy - Vx, set VF = NOT borrow.

    }
    fn op_8xyE(&mut self) { //8xyE - SHL Vx {, Vy}. Set Vx = Vx SHL 1.

    }
    fn op_9xy0(&mut self) { //SNE Vx, Vy. Skip next instruction if Vx != Vy.

    }
    fn op_annn(&mut self) { //LD I, addr. Set I = nnn

    }
    fn op_bnnn(&mut self) { //JP V0, addr. Jump to location nnn + V0

    }
    fn op_cxkk(&mut self) { //RND Vx, byte. Set Vx = random byte AND kk

    }
    fn op_dxyn(&mut self) { //DRW Vx, Vy, nibble. Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

    }
    fn op_ex9e(&mut self) { //SKP Vx Skip next instruction if key with the value of Vx is pressed.

    }
    fn op_exa1(&mut self) { //SKNP Vx. Skip next instruction if key with the value of Vx is not pressed.

    }
    fn op_fx07(&mut self) { //LD Vx, DT. Set Vx = delay timer value.

    }
    fn op_fx0a(&mut self) {// LD Vx, K. Wait for a key press, store the value of the key in Vx.

    }
    fn op_fx15(&mut self) {//LD DT, Vx. Set delay timer = Vx.

    }
    fn op_fx18(&mut self) {//LD ST, Vx Set sound timer = Vx.

    }
    fn op_fx1e(&mut self) {//ADD I, Vx. Set I = I + Vx.

    }
    fn op_fx29(&mut self) {//LD F, Vx. Set I = location of sprite for digit Vx.

    }
    fn op_fx33(&mut self) {//LD B, Vx. Store BCD representation of Vx in memory locations I, I+1, and I+2.

    }
    fn op_fx55(&mut self) {//LD [I], Vx. Store registers V0 through Vx in memory starting at location I.

    }
    fn op_fx65(&mut self) {//LD Vx, [I]. Read registers V0 through Vx from memory starting at location I.

    }
    pub fn chip8_says_hello(&self){
        println!("Chip 8 says hello");
    }
}

pub fn new_chip8() -> Chip8 {
   let mut new_chip = Chip8 {
       v: [0; 16],
       mem: [0; 4096],
       opcode: 0,
       i: 0,
       pc: 0,
       stack: [0; 16],
       sp: 0,
       delay: 0,
       sound: 0,
       gfx: [0; 32 * 64]
   };
    new_chip.init();
    return new_chip;
}












