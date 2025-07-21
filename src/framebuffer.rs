use image::{ImageBuffer, Rgba};
use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub scale: u32,
    buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, scale: u32) -> Self {
        let buffer = ImageBuffer::from_pixel(width, height, Rgba([0, 0, 0, 255])); 
        Self {
            width,
            height,
            scale,
            buffer,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 255]); // celda muerta
        }
    }

    pub fn point(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            self.buffer.put_pixel(x, y, Rgba([color.r, color.g, color.b, color.a]));
        }
    }

    pub fn get_color(&self, x: i32, y: i32) -> Color {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return Color::BLACK;
        }
        let pixel = self.buffer.get_pixel(x as u32, y as u32);
        Color::new(pixel[0], pixel[1], pixel[2], pixel[3])
    }

    pub fn draw_to_raylib(&self, d: &mut RaylibDrawHandle) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.buffer.get_pixel(x, y);
                if pixel[0] > 0 || pixel[1] > 0 || pixel[2] > 0 {
                    d.draw_rectangle(
                        (x * self.scale) as i32,
                        (y * self.scale) as i32,
                        self.scale as i32,
                        self.scale as i32,
                        Color::WHITE,
                    );
                }
            }
        }
    }
}