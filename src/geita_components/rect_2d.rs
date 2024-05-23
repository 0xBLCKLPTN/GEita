use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use std::path::Path;

use super::camera_2d::Camera2D;
use super::GEitaComponentsEnum;

pub struct Rect2D<'a> {
    pub canvas: &'a mut Canvas<sdl2::video::Window>,
    pub position: [i32; 2],
    pub texture: Texture<'a>,
    pub size: [usize; 2],
    pub child_components: Vec<GEitaComponentsEnum>,
}

impl<'a> Rect2D<'a> {
    pub fn draw_rainbow(
        canvas: &'a mut Canvas<sdl2::video::Window>,
        position: &mut [i32; 2],
        texture_creator: &'a TextureCreator<WindowContext>,
        size: &mut [usize; 2],
        camera: &Camera2D,
    ) -> Self {
        let mut child_components: Vec<GEitaComponentsEnum> = Vec::new();
        let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, 512, 512)
            .unwrap();
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..256 {
                    for x in 0..256 {
                        let offset = y * pitch + x * 3;
                        buffer[offset + 0] = x as u8;
                        buffer[offset + 1] = y as u8;
                        buffer[offset + 2] = 0;
                    }
                }
            })
            .unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        let rect1 = Rect::new(position[0], position[1], 250, 250);
        canvas.copy(&texture, None, rect1).unwrap();

        Rect2D {
            canvas,
            position: *position,
            texture,
            size: *size,
            child_components,
        }
    }

    pub fn draw(
        canvas: &'a mut Canvas<sdl2::video::Window>,
        position: &mut [i32; 2],
        texture_creator: &'a TextureCreator<WindowContext>,
        size: &mut [usize; 2],
    ) -> Self {
        let mut child_components: Vec<GEitaComponentsEnum> = Vec::new();
        let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, 512, 512)
            .unwrap();

        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..size[0] {
                    for x in 0..size[1] {
                        let offset = y * pitch + x * 3;
                        buffer[offset + 0] = 255 as u8;
                        buffer[offset + 1] = 255 as u8;
                        buffer[offset + 2] = 255 as u8;
                    }
                }
            })
            .unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        let rect1 = Rect::new(position[0], position[1], 250, 250);
        canvas.copy(&texture, None, rect1).unwrap();

        Rect2D {
            canvas,
            position: *position,
            texture,
            size: *size,
            child_components,
        }
    }
    pub fn add_component(&mut self, component: GEitaComponentsEnum) {
        self.child_components.push(component)
    }
}
