use std::env;
use std::fs;

use sfml::{
    graphics::{
        Color, RenderTarget, RenderWindow, RectangleShape, Shape, Transformable, Drawable,
    },
    window::{ContextSettings, Event, Style},
};

pub const SCALE : f32 = 20.;
pub const WIDTH: f32 = 64.;
pub const HEIGHT: f32 = 32.;
pub const START_PROGRAM: u16 = 0x200;

struct Chip8 {
    ram:[u8;4096],
    registers:[u8;16],
    index_register:u16,
    delay_register:u8,
    sound_register:u8,
    program_counter:u16,
    stack:[u16;16],
    stack_pointer:u8,
    video_memory:[[bool;WIDTH as usize];HEIGHT as usize]
}
impl Chip8 {
    fn new() -> Chip8 {
        Chip8 {
            ram:[0;4096],
            registers:[0;16],
            index_register:0,
            delay_register:0,
            sound_register:0,
            program_counter:START_PROGRAM,
            stack:[0;16],
            stack_pointer:0,
            video_memory:[[false;WIDTH as usize];HEIGHT as usize]
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
    fn draw(&self, rw: &mut RenderWindow) {
        rw.clear(Color::BLACK);
        for i in 0..HEIGHT as usize {
            for j in 0..WIDTH as usize {
                let mut rect = RectangleShape::new();
                rect.set_outline_color(Color { r: 100, g: 100, b: 100, a: 255 });
                rect.set_outline_thickness(2.);
                rect.set_size((SCALE, SCALE));
                rect.set_fill_color( if self.video_memory[i][j] { Color::WHITE } else { Color::BLACK } );
                rect.set_position((j as f32*SCALE, i as f32*SCALE));
                rw.draw(&rect);
            }
        }
    } 
    
    fn call_operation(&mut self, _rw: &mut RenderWindow) {
        match self.get_opcode() {
            0x00E0 => {
                for i in 0..HEIGHT as usize {
                    for j in 0..WIDTH as usize {
                        self.video_memory[i][j] = false;
                    }
                }
                self.inc_program_counter();
            }
            0x00EE => {
                self.stack_pointer -= 1;
                self.inc_program_counter();
            }
            0x1000..=0x1FFF => {
                self.program_counter = self.get_opcode() & 0x0FFF;
            }
            0x2000..=0x2FFF => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.program_counter = self.get_opcode() & 0x0FFF;
            }
            0x3000..=0x3FFF => {
                if self.get_second_octet() == 0xF {
                    panic!("Register out of range");
                }
                if self.registers[self.get_second_octet() as usize] == (self.get_opcode()&0x00FF) as u8 {
                    self.inc_program_counter();
                }
                self.inc_program_counter();
            }
            0x4000..=0x4FFF => {
                if self.get_second_octet() == 0xF {
                    panic!("Register out of range");
                }
                if self.registers[self.get_second_octet() as usize] != (self.get_opcode()&0x00FF) as u8 {
                    self.inc_program_counter();
                }
                self.inc_program_counter();
            }
            0x5000..=0x5FFF => {
                if self.get_second_octet() == 0xF || self.get_third_octet() == 0xF {
                    panic!("Register out of range");
                }
                if self.registers[self.get_second_octet() as usize] == self.registers[self.get_third_octet() as usize] {
                    self.inc_program_counter();
                }
                self.inc_program_counter();
            }
            0x6000..=0x6FFF => {
                if self.get_second_octet() != 0xF {
                    self.registers[self.get_second_octet() as usize] = (self.get_opcode()&0x00FF) as u8;
                    self.inc_program_counter();
                }
            }
            0x7000..=0x7FFF => {
                if self.get_second_octet() == 0xF {
                    panic!("Register out of range");
                }
                self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize].wrapping_add((self.get_opcode()&0xFF) as u8);
                self.inc_program_counter();
            }
            0x8000..=0x8FFF => {
                if self.get_second_octet() == 0xF || self.get_third_octet() == 0xF {
                    panic!("Register out of range");
                }
                match self.get_fourth_octet() {
                    0x0 => {
                        self.registers[self.get_second_octet() as usize] = self.registers[self.get_third_octet() as usize];
                    }
                    0x1 => {
                        self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize] | self.registers[self.get_third_octet() as usize];
                    }
                    0x2 => {
                        self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize] & self.registers[self.get_third_octet() as usize];
                    }
                    0x3 => {
                        self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize] ^ self.registers[self.get_third_octet() as usize];
                    }
                    0x4 => {
                        let sum = self.registers[self.get_second_octet() as usize] as u16 + self.registers[self.get_third_octet() as usize] as u16;
                        if sum > 0xFF {
                            self.registers[0xF] = 1;
                        } else {
                            self.registers[0xF] = 0;
                        }
                        self.registers[self.get_second_octet() as usize] = sum as u8;
                    }
                    0x5 => {
                        if self.registers[self.get_second_octet() as usize] > self.registers[self.get_third_octet() as usize] {
                            self.registers[0xF] = 1;
                            self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize] - self.registers[self.get_third_octet() as usize];
                        } else {
                            self.registers[0xF] = 0;
                            self.registers[self.get_second_octet() as usize] = 0;
                        }
                    }
                    0x6 => {
                        self.registers[0xF] = self.registers[self.get_second_octet() as usize] & 0x1;
                        self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize] >> 1;
                    }
                    0x7 => {
                        if self.registers[self.get_third_octet() as usize] > self.registers[self.get_second_octet() as usize] {
                            self.registers[0xF] = 1;
                        } else {
                            self.registers[0xF] = 0;
                        }
                        self.registers[self.get_second_octet() as usize] = self.registers[self.get_third_octet() as usize] - self.registers[self.get_second_octet() as usize];
                    }
                    0xE => {
                        self.registers[0xF] = self.registers[self.get_second_octet() as usize] & 0x80;
                        self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize] << 1;
                    }
                    _ => { println!("Opcode not implemented: {:X}", self.get_opcode()) }
                }
                self.inc_program_counter();
            }
            0x9000..=0x9FFF => {
            }
            0xA000..=0xAFFF => {
                self.index_register = self.get_opcode() & 0x0FFF;
                self.inc_program_counter();
            }
            0xB000..=0xBFFF => {
                self.program_counter = self.registers[self.get_second_octet() as usize ] as u16 + ( self.get_opcode() & 0x0FFF );

            }
            0xD000..=0xDFFF => {
                self.registers[0xF] = 0;
                let x = self.registers[self.get_second_octet() as usize]%WIDTH as u8;
                let y = self.registers[self.get_third_octet() as usize]%HEIGHT as u8;
                let n = self.get_fourth_octet();
                for i in 0..n {
                    let byte = self.ram[(self.index_register+i as u16) as usize];
                    for j in 0..8 {
                        if (byte >> (7-j)) & 0x1 == 1 {
                            if self.video_memory[(i+y) as usize][(j+x) as usize] {
                                self.registers[0xF] = 1;
                                self.video_memory[(i+y) as usize][(j+x) as usize] = false;
                            } else {
                                self.video_memory[(i+y) as usize][(j+x) as usize] = true;
                            }
                        }
                    }
                }
                self.inc_program_counter();
            }
            0xF000..=0xFFFF => {
                if self.get_second_octet() == 0xF {
                    panic!("Register out of range");
                }
                match self.get_opcode() & 0x00FF {
                    0x07 => {
                        self.registers[self.get_second_octet() as usize] = self.delay_register;
                    }
                    0x0A => {
                        let mut key_pressed = false;
                        println!( "Keypress? TODO");
                    }
                    0x15 => {
                        self.delay_register = self.registers[self.get_second_octet() as usize];
                    }
                    0x18 => {
                        self.sound_register = self.registers[self.get_second_octet() as usize];
                    }
                    0x1E => {
                        self.index_register += self.registers[self.get_second_octet() as usize] as u16;
                    }
                    0x29 => {
                        self.index_register = ( 0x50 + ( self.get_second_octet() * 5 ) ) as u16;
                    }
                    0x33 => {
                        self.ram[self.index_register as usize] = self.registers[self.get_second_octet() as usize] / 100;
                        self.ram[self.index_register as usize+1] = (self.registers[self.get_second_octet() as usize] / 10) % 10;
                        self.ram[self.index_register as usize+2] = self.registers[self.get_second_octet() as usize] % 10;
                    }
                    0x55 => {
                        for i in 0..self.get_second_octet()+1 {
                            self.ram[(self.index_register+i as u16) as usize] = self.registers[i as usize];
                        }
                    }
                    0x65 => {
                        for i in 0..self.get_second_octet()+1 {
                            self.registers[i as usize] = self.ram[(self.index_register+i as u16) as usize];
                        }
                    }
                    _ => { println!("Opcode not implemented: {:X}", self.get_opcode()) }
                }
                self.inc_program_counter();
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom = fs::read(args[1].clone()).expect("Unable to read file");
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
    let mut Chip8 = Chip8::new();
    for (pos, e) in font.iter().enumerate() {
        Chip8.ram[0x50 as usize + pos] = *e;
    }
    for (pos, e) in rom.iter().enumerate() {
        Chip8.ram[START_PROGRAM as usize + pos] = *e;
    }
    let mut rw = RenderWindow::new(
        ((WIDTH*SCALE) as u32, (HEIGHT*SCALE) as u32),
        "Chip8",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    rw.set_vertical_sync_enabled(true);
    rw.clear(Color::BLACK);
    while rw.is_open() {
        while let Some(ev) = rw.poll_event() {
            match ev {
                Event::Closed => rw.close(),
                _ => {}
            }
        }
        Chip8.call_operation(&mut rw);
        Chip8.draw(&mut rw);
        rw.display();
    }
}