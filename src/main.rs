mod chip8;
use std::env;
use std::fs;

use sfml::{
    graphics::{
        Color, RenderTarget, RenderWindow, Texture
    },
    window::{ContextSettings, Event, Style},
    system::{Clock, Time},
    audio::{SoundBuffer, Sound},
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
    chip_8.load_font();
    
    chip_8.load_rom(rom);
    let mut texture:SfBox<Texture> = Texture::new().unwrap();
    let mut rw = RenderWindow::new(
        ((WIDTH*SCALE) as u32, (HEIGHT*SCALE) as u32),
        "Chip8",
        Style::CLOSE,
        &ContextSettings::default(),
    );

    let mut samples = [0i16; 44100];
    for i in 0..44100 {
        samples[i] = ( (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / 44100.0).sin() * (i16::MAX as f32) ) as i16;
    }
    let sbuffer = SoundBuffer::from_samples(&samples, 1, 44100).unwrap();
    let mut sound = Sound::with_buffer(&sbuffer);
    sound.set_looping(true);
    let mut clock = Clock::start();
    let mut time: Time;

    rw.set_vertical_sync_enabled(true);
    rw.clear(Color::BLACK);
    while rw.is_open() {
        while let Some(ev) = rw.poll_event() {
            match ev {
                Event::Closed => rw.close(),
                Event::KeyPressed {code, ..} => {
                    chip_8.key_pressed(code);
                },
                Event::KeyReleased {code, ..} => {
                    chip_8.key_released(code);
                },
                _ => {}
            }
        }
        chip_8.call_operation(&mut rw);
        chip_8.video.render(&mut rw, &mut texture);

        time = clock.elapsed_time();
        if time.as_seconds() >= 1.0/60.0 {
            clock.restart();
            chip_8.update_timers(&mut sound);
        }
    }
}