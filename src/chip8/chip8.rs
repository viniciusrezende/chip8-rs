use sfml::graphics::RenderWindow;

use crate::chip8::video::Video;
use crate::START_PROGRAM;
use crate::HEIGHT;
use crate::WIDTH;

pub struct Chip8 {
    pub ram:[u8;4096],
    registers:[u8;16],
    index_register:u16,
    delay_register:u8,
    sound_register:u8,
    program_counter:u16,
    stack:[u16;16],
    stack_pointer:u8,
    pub video:Video
}
impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            ram:[0;4096],
            registers:[0;16],
            index_register:0,
            delay_register:0,
            sound_register:0,
            program_counter:START_PROGRAM,
            stack:[0;16],
            stack_pointer:0,
            video:Video::new()
        }
    }
    pub fn load_font(&mut self) {
        let font:[u8;80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0,
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
        for (pos, e) in font.iter().enumerate() {
            self.ram[0x50 as usize + pos] = *e;
        }
    }
    pub fn load_rom(&mut self, rom:Vec<u8>) {
        for i in 0..rom.len() {
            self.ram[START_PROGRAM as usize + i] = rom[i];
        }
    }
    fn get_opcode(&self) -> u16 {
        ((self.ram[self.program_counter as usize] as u16) << 8) | self.ram[self.program_counter as usize+1] as u16
    }
    fn get_first_octet(&self) -> u8 {
        self.ram[self.program_counter as usize] >> 4
    }
    fn get_second_octet(&self) -> u8 {
        self.ram[self.program_counter as usize] & 0x0F
    }
    fn get_third_octet(&self) -> u8 {
        self.ram[self.program_counter as usize+1] >> 4
    }
    fn get_fourth_octet(&self) -> u8 {
        self.ram[self.program_counter as usize+1] & 0x0F
    }

    fn op_00e0(&mut self) {
        self.video.clear();
        self.inc_program_counter();
    }
    fn op_00ee(&mut self) {
        self.program_counter = self.stack[self.stack_pointer as usize];
        self.stack_pointer -= 1;
        self.inc_program_counter();
    }
    fn table_0(&mut self) {
        match self.get_opcode() & 0x00FF {
            0xE0 => self.op_00e0(),
            0xEE => self.op_00ee(),
            _ => { println!("Opcode not implemented: {:X}", self.get_opcode()) }
        }
    }
    fn op_1nnn(&mut self) {
        self.program_counter = self.get_opcode() & 0x0FFF;
    }
    fn op_2nnn(&mut self) {
        self.stack_pointer += 1;
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.program_counter = self.get_opcode() & 0x0FFF;
    }
    fn op_3xkk(&mut self) {
        if self.get_second_octet() == 0xF {
            panic!("Register out of range");
        }
        if self.registers[self.get_second_octet() as usize] == (self.get_opcode()&0x00FF) as u8 {
            self.inc_program_counter();
        }
        self.inc_program_counter();
    }
    fn op_4xkk(&mut self) {
        if self.get_second_octet() == 0xF {
            panic!("Register out of range");
        }
        if self.registers[self.get_second_octet() as usize] != (self.get_opcode()&0x00FF) as u8 {
            self.inc_program_counter();
        }
        self.inc_program_counter();
    }
    fn op_5xy0(&mut self) {
        if self.get_second_octet() == 0xF || self.get_third_octet() == 0xF {
            panic!("Register out of range");
        }
        if self.registers[self.get_second_octet() as usize] == self.registers[self.get_third_octet() as usize] {
            self.inc_program_counter();
        }
        self.inc_program_counter();
    }
    fn op_6xkk(&mut self) {
        if self.get_second_octet() != 0xF {
            self.registers[self.get_second_octet() as usize] = (self.get_opcode()&0x00FF) as u8;
            self.inc_program_counter();
        }
    }
    fn op_7xkk(&mut self) {
        if self.get_second_octet() == 0xF {
            panic!("Register out of range");
        }
        self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize].wrapping_add((self.get_opcode()&0xFF) as u8);
        self.inc_program_counter();
    }
    fn op_8xy0(&mut self) {
        self.registers[self.get_second_octet() as usize] = self.registers[self.get_third_octet() as usize];
    }
    fn op_8xy1(&mut self) {
        self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize] | self.registers[self.get_third_octet() as usize];
    }
    fn op_8xy2(&mut self) {
        self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize] & self.registers[self.get_third_octet() as usize];
    }
    fn op_8xy3(&mut self) {
        self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize] ^ self.registers[self.get_third_octet() as usize];
    }
    fn op_8xy4(&mut self) {
        let sum = self.registers[self.get_second_octet() as usize] as u16 + self.registers[self.get_third_octet() as usize] as u16;
        if sum > 0xFF {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[self.get_second_octet() as usize] = sum as u8;
    }
    fn op_8xy5(&mut self) {
        if self.registers[self.get_second_octet() as usize] > self.registers[self.get_third_octet() as usize] {
            self.registers[0xF] = 1;
            self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize] - self.registers[self.get_third_octet() as usize];
        } else {
            self.registers[0xF] = 0;;
            self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize].overflowing_sub(self.registers[self.get_third_octet() as usize]).0;
        }
    }
    fn op_8xy6(&mut self) {
        self.registers[0xF] = self.registers[self.get_second_octet() as usize] & 0x1;
        self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize] >> 1;
    }
    fn op_8xy7(&mut self) {
        if self.registers[self.get_third_octet() as usize] > self.registers[self.get_second_octet() as usize] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[self.get_second_octet() as usize] = self.registers[self.get_third_octet() as usize] - self.registers[self.get_second_octet() as usize];
    }
    fn op_8xye(&mut self) {
        self.registers[0xF] = self.registers[self.get_second_octet() as usize] & 0x80;
        self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize] << 1;
    }

    fn table_8(&mut self) {
        if self.get_second_octet() == 0xF || self.get_third_octet() == 0xF {
            panic!("Register out of range");
        }
        match self.get_fourth_octet() {
            0x0 => self.op_8xy0(),
            0x1 => self.op_8xy1(),
            0x2 => self.op_8xy2(),
            0x3 => self.op_8xy3(),
            0x4 => self.op_8xy4(),
            0x5 => self.op_8xy5(),
            0x6 => self.op_8xy6(),
            0x7 => self.op_8xy7(),
            0xE => self.op_8xye(),
            _ => { println!("Opcode not implemented: {:X}", self.get_opcode()) }
        }
        self.inc_program_counter();
    }
    fn op_9xy0(&mut self) {
        if self.registers[self.get_second_octet() as usize] != self.registers[self.get_third_octet() as usize] {
            self.inc_program_counter();
        }
        self.inc_program_counter();
    }
    fn op_annn(&mut self) {
        self.index_register = self.get_opcode() & 0x0FFF;
        self.inc_program_counter();
    }
    fn op_bnnn(&mut self) {
        self.program_counter = self.registers[0] as u16 + ( self.get_opcode() & 0x0FFF );
        self.inc_program_counter();
    }
    fn op_cxkk(&mut self) {
        println!("Opcode not implemented: {:X}", self.get_opcode());
    }
    fn op_dxyn(&mut self) {
        self.registers[0xF] = 0;
        let x = self.registers[self.get_second_octet() as usize]%WIDTH as u8;
        let y = self.registers[self.get_third_octet() as usize]%HEIGHT as u8;
        let n = self.get_fourth_octet();
        for i in 0..n {
            let byte = self.ram[(self.index_register+i as u16) as usize];
            for j in 0..8 {
                if (byte >> (7-j)) & 0x1 == 1 {
                    self.video.update((x+j) as u32, (y+i) as u32);
                }
            }
        }
        self.inc_program_counter();
    }

    fn op_exa1(&mut self) {
        println!("Opcode not implemented: {:X}", self.get_opcode());
    }
    fn op_ex9e(&mut self) {
        println!("Opcode not implemented: {:X}", self.get_opcode());
    }
    fn table_e(&mut self) {
        match self.get_opcode() & 0x00FF {
            0x9E => {
                self.op_ex9e();
            }
            0xA1 => {
                self.op_exa1();
            }
            _ => { println!("Opcode not implemented: {:X}", self.get_opcode()) }
        }
        self.inc_program_counter();
    }

    fn op_fx07(&mut self) {
        self.registers[self.get_second_octet() as usize] = self.delay_register;
    }
    fn op_fx0a(&mut self) {
        let _key_pressed = false;
        println!( "Keypress? TODO");
    }
    fn op_fx15(&mut self) {
        self.delay_register = self.registers[self.get_second_octet() as usize];
    }
    fn op_fx18(&mut self) {
        self.sound_register = self.registers[self.get_second_octet() as usize];
    }
    fn op_fx1e(&mut self) {
        self.index_register += self.registers[self.get_second_octet() as usize] as u16;
    }
    fn op_fx29(&mut self) {
        self.index_register = ( 0x50 + ( self.get_second_octet() * 5 ) ) as u16;
    }
    fn op_fx33(&mut self) {
        self.ram[self.index_register as usize] = self.registers[self.get_second_octet() as usize] / 100;
        self.ram[self.index_register as usize+1] = (self.registers[self.get_second_octet() as usize] / 10) % 10;
        self.ram[self.index_register as usize+2] = self.registers[self.get_second_octet() as usize] % 10;
    }
    fn op_fx55(&mut self) {
        for i in 0..self.get_second_octet()+1 {
            self.ram[(self.index_register+i as u16) as usize] = self.registers[i as usize];
        }
    }
    fn op_fx65(&mut self) {
        for i in 0..self.get_second_octet()+1 {
            self.registers[i as usize] = self.ram[(self.index_register+i as u16) as usize];
        }
    }
    fn table_f(&mut self) {
        if self.get_second_octet() == 0xF {
            panic!("Register out of range");
        }
        match self.get_opcode() & 0x00FF {
            0x07 => self.op_fx07(),
            0x0A => self.op_fx0a(),
            0x15 => self.op_fx15(),
            0x18 => self.op_fx18(),
            0x1E => self.op_fx1e(),
            0x29 => self.op_fx29(),
            0x33 => self.op_fx33(),
            0x55 => self.op_fx55(),
            0x65 => self.op_fx65(),
            _ => { println!("Opcode not implemented: {:X}", self.get_opcode()) }
        }
        self.inc_program_counter();
    }

    pub fn call_operation(&mut self, _rw: &mut RenderWindow) {
        match self.get_first_octet() {
            0x0 => {
                self.table_0();
            }
            0x1 => {
                self.op_1nnn();
            }
            0x2 => {
                self.op_2nnn();
            }
            0x3 => {
                self.op_3xkk();
            }
            0x4 => {
                self.op_4xkk();
            }
            0x5 => {
                self.op_5xy0();
            }
            0x6 => {
                self.op_6xkk();
            }
            0x7 => {
                self.op_7xkk();
            }
            0x8 => {
                self.table_8();
            }
            0x9 => {
                self.op_9xy0();
            }
            0xA => {
                self.op_annn();
            }
            0xB => {
                self.op_bnnn();
            }
            0xC => {
                self.op_cxkk();
            }
            0xD => {
                self.op_dxyn();
            }
            0xE => {
               self.table_e(); 
            }
            0xF => {
               self.table_f(); 
            }
            _=> {
                println!("Opcode not implemented: {:X}", self.get_opcode());
            }
        }
        
        
    }
    fn inc_program_counter(&mut self) {
        self.program_counter += 2;
    }
}