mod chip8;
use std::env;
use std::fs;

use sfml::{
    graphics::{
        Color, RenderTarget, RenderWindow, Texture
    },
    window::{ContextSettings, Event, Style},
};
use sfml::SfBox;
use chip8::chip8::Chip8;
pub const SCALE : f32 = 20.;
pub const WIDTH: f32 = 64.;
pub const HEIGHT: f32 = 32.;
pub const START_PROGRAM: u16 = 0x200;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom = fs::read(args[1].clone()).expect("Unable to read file");
    let mut chip_8 = Chip8::new();
    chip_8.load_rom(rom);
    chip_8.load_font();

    let mut texture:SfBox<Texture> = Texture::new().unwrap();
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
        chip_8.call_operation(&mut rw);
        chip_8.video.render(&mut rw, &mut texture);
        rw.display();
    }
}