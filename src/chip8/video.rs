use crate::SCALE;
use crate::WIDTH;
use crate::HEIGHT;

use sfml::{
    graphics::{
        Color, RenderTarget, RenderWindow, Sprite, Texture, Transformable, Image, Rect
    },
    system::{Vector2f},
    SfBox
};
pub struct Video {
    video_memory: Vec<Vec<bool>>,
}
impl Video {
    pub fn new() -> Video {
        Video {
            video_memory: vec![vec![false; WIDTH as usize]; HEIGHT as usize]
        }
    }
    pub fn update(&mut self, x:u32, y:u32) {
        let pixel = self.video_memory[y as usize][x as usize];
        if pixel {
            self.video_memory[y as usize][x as usize] = false;
        } else {
            self.video_memory[y as usize][x as usize] = true;
        }
    }
    pub fn clear(&mut self) {
        self.video_memory = vec![vec![false; WIDTH as usize]; HEIGHT as usize];
    }
    pub fn render(&mut self, rw:&mut RenderWindow, texture: &mut SfBox<Texture>) {
        let mut image = Image::new(WIDTH as u32, HEIGHT as u32);
        unsafe {
            for y in 0..HEIGHT as u32 {
                for x in 0..WIDTH as u32 {
                    let pixel = self.video_memory[y as usize][x as usize];
                        if pixel {
                            image.set_pixel(x, y, Color::WHITE);
                        } else {
                            image.set_pixel(x, y, Color::BLACK);
                        }
                }
            }
            texture.load_from_image(&image, Rect { left: 0, top: 0, width: WIDTH as i32, height: HEIGHT as i32 }).unwrap();
            let mut spr = Sprite::with_texture(&texture);
            spr.set_scale(Vector2f::new(SCALE as f32, SCALE as f32));
            rw.draw(&spr);
        }
    }
}