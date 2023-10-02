use std::env;
use std::fs;
use hex;
use sfml::{
    system::{Clock, Time},
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
        (self.ram[self.program_counter as usize] >> 4)
    }
    fn get_second_octet(&self) -> u8 {
        (self.ram[self.program_counter as usize] & 0x0F)
    }
    fn get_third_octet(&self) -> u8 {
        (self.ram[self.program_counter as usize+1] >> 4)
    }
    fn get_fourth_octet(&self) -> u8 {
        (self.ram[self.program_counter as usize+1] & 0x0F)
    }
    fn draw(&self, rw: &mut RenderWindow) {
        rw.clear(Color::BLACK);
        for i in 0..HEIGHT as usize {
            for j in 0..WIDTH as usize {
                //if self.video_memory[i][j] {
                    let mut rect = RectangleShape::new();
                    rect.set_outline_color(Color { r: 100, g: 100, b: 100, a: 255 });
                    rect.set_outline_thickness(2.);
                    rect.set_size((SCALE, SCALE));
                    rect.set_fill_color( if (self.video_memory[i][j]) { Color::WHITE } else { Color::BLACK } );
                    rect.set_position((j as f32*SCALE, i as f32*SCALE));
                    rw.draw(&rect);
                //}
            }
        }
    } 
    
    fn call_operation(&mut self, rw: &mut RenderWindow) {
        match self.get_opcode() {
            0x00E0 => {
                for i in 0..HEIGHT as usize {
                    for j in 0..WIDTH as usize {
                        self.video_memory[i][j] = false;
                    }
                }
                self.inc_program_counter();
            }
            0x1000..=0x1FFF => {
                self.program_counter = self.get_opcode() & 0x0FFF;
            }
            0x6000..=0x6FFF => {
                if self.get_second_octet() == 0xF {
                    panic!("Register out of range");
                }
                self.registers[self.get_second_octet() as usize] = (self.get_opcode()&0x00FF) as u8;
                self.inc_program_counter();
            }
            0x7000..=0x7FFF => {
                if self.get_second_octet() == 0xF {
                    panic!("Register out of range");
                }
                self.registers[self.get_second_octet() as usize] = self.registers[self.get_second_octet() as usize]+(self.get_opcode()&0x00FF) as u8;
                self.inc_program_counter();
            }
            0xA000..=0xAFFF => {
                self.index_register = self.get_opcode() & 0x0FFF;
                self.inc_program_counter();
            }
            0xD000..=0xDFFF => {
                self.registers[0xF] = 0;
                let x = self.registers[self.get_second_octet() as usize]%WIDTH as u8;
                let y = self.registers[self.get_third_octet() as usize]%HEIGHT as u8;
                let n = self.get_fourth_octet();
                for i in 0..n {
                    let byte = self.ram[(self.index_register+i as u16) as usize];
                    println!("x {} y {} byte: {:X}, index {}, ramz {} n {}", x, y, byte, self.index_register+i as u16,self.ram[(self.index_register+i as u16) as usize], n);
                    for j in 0..8 {
                        if (byte >> 7-j) & 0x1 == 1 {
                            if self.video_memory[(i+y) as usize][(j+x) as usize] == true {
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
            _=> {
                println!("Opcode not implemented: {:X}", self.get_opcode());
            }
        }
        self.ram[self.program_counter as usize+1];
        
        
    }
    fn inc_program_counter(&mut self) {
        self.program_counter += 2;
    }
}
/**
    todo:
    CHIP-8 INSTRUCTION SET
    Stored Code Mnemonic Description
    0000 NOP No Operation.
    00EE RETURN Return from Subroutine.
    1MMM GOTO MMM Jump to location MMM.
    2MMM DO MMM Call Subroutine.
    3XKK SKF VX=KK Skip next Instruction if VX=KK.
    4XKK SKF VX≠KK Skip next Instruction if VX≠KK.
    5XY0 SKF VX=VY Skip next Instruction if VX=VY.
    6XKK VX=KK Assign Hex value KK to Register VX.
    7XKK VX=VX+KK Add KK to VX.
    8XY0 VX=VY Copy VY to VX.
    8XY1 VX=VX│VY Logical OR VX with VY.
    8XY2 VX=VX.VY Logical AND VX with VY.
    8XY3 VX=VX XOR VY Logical XOR VX with VY.
    8XY4 VX=VX+VY Add VY to VX.If result >FF, then VF=1.
    8XY5 VX=VX-VY Subtract VY. If VX<VY, then VF=0.
    9XY0 SKF VX≠VY Skip next Instruction if VX≠VY.
    AMMM I=MMM Set memory Index Pointer to MMM.
    BMMM GOTO MMM+V0 Jump to location MMM+V0.
    CXKK VX=RND.KK Get random byte, then AND with KK.
    DXYN SHOW N@VX,VY Display N-byte pattern at (VX,VY).
    EX9E SKF VX=KEY Skip if key down =VX. No wait.
    EXA1 SKF VX≠KEY Skip if key down ≠VX. No wait.
    F000 STOP Jump to Monitor (CHIPOS).
    FX07 VX=TIME Get current timer value.
    FX0A VX=KEY Input Hex key code. Wait for key down.
    FX15 TIME=VX Initialize Timer. 01=20 mS.
    FX17 PITCH=VX Set the Pitch of the Tone Generator to VX.
    FX18 TONE=VX Sound Tone for 20 timesVX milliseconds.
    FX1E I=I+VX Add VX to Memory Pointer.
    FX29 I=DSP,VX Set Pointer to show VX (LS digit).
    FX33 MI=DEQ,VX Store 3 digit decimal equivalent of VX.
    FX55 MI=VO:VX Store V0 through VX at I. I=I+X+1.
    FX65 V0:VX=MI Load V0 through VX at I. I=I+X+1.
    FX70 RS485=VX Send data in VX to RS485 Port.
    FX71 VX=RS485 Waits for received RS485 data. Place in VX.
    FX72 BAUD=VX Set RS485 Baud rate.
    */
fn main() {
    let args: Vec<String> = env::args().collect();
    let rom = fs::read(args[1].clone()).expect("Unable to read file");
    let mut buff:[u8;2] = [0;2];
    let mut Chip8 = Chip8::new();
    for (pos, e) in rom.iter().enumerate() {
        Chip8.ram[START_PROGRAM as usize + pos] = *e;
        if pos%2 == 0 && pos != 0 {
            println!("{}", hex::encode(buff));
        }
        buff[pos%2] = *e;
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