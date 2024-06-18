use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use std::path::Path;

use super::camera_2d::Camera2D;
use super::GEitaComponentsEnum;


pub struct Image2D<'a> {
    pub parent_component: Option<GEitaComponentsEnum>,
    pub canvas: &'a mut Canvas<sdl2::video::Window>,
    pub position: [i32; 2],
    pub path: &'a Path,
    pub texture: Texture<'a>,
    pub size: [u32; 2],
}

impl<'a> Image2D<'a> {
    /* Component that render 2D image like as png or jpg (jpeg).
    As dependency, component used libsdl2-image-dev.
     
    Usage Example:
    ```rust
        Image2D::draw(
            &mut window.canvas,
            &mut [120i32; 2], // position on canvas.
            &window.texture_creator,
            grass_tile, // Path
        );
    ```
    
    */
    pub fn draw(
        canvas: &'a mut Canvas<sdl2::video::Window>,
        position: &mut [i32; 2],
        texture_creator: &'a TextureCreator<WindowContext>,
        png: &'a Path,
        size: &mut [u32; 2],
        parent_component: Option<GEitaComponentsEnum>
    ) -> Self {
        let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
        let texture = texture_creator.load_texture(png).unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));

        let rect1 = Rect::new(
            position[0],
            position[1],
            size[0],
            size[1],
        );
        canvas.copy(&texture, None, rect1).unwrap();

        Image2D {
            parent_component,
            canvas,
            position: *position,
            path: png,
            texture,
            size: *size,
        }
    }
}
