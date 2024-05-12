use std::path::Path;
use sdl2::render::{Texture, Canvas, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use crate::geita_components::GeitaComponents;

pub struct ImageComp<'a> {
    pub canvas: &'a mut Canvas<sdl2::video::Window>,
    pub position: [i32; 2],
    pub path: &'a Path,
    pub texture: Texture<'a>,
    pub child_components: Option<GeitaComponents>,
}

impl<'a> ImageComp<'a> {
    pub fn draw(
        canvas: &'a mut Canvas<sdl2::video::Window>,
        position: &mut [i32; 2],
        texture_creator: &'a TextureCreator<WindowContext>,
        png: &'a Path,
    ) -> Self {
        let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
        let texture = texture_creator.load_texture(png).unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        let rect1 = Rect::new(position[0], position[1], 250, 250);
        canvas.copy(&texture, None, rect1).unwrap();

        ImageComp {
            canvas,
            position: *position,
            path: png,
            texture,
            child_components: None,
        }
    }
}
