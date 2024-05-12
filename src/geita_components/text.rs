use crate::Font;
use crate::Canvas;
use crate::Texture;

pub struct Text<'a> {
    pub text: String,
    pub font: Font<'a, 'a>,
    pub size: u16,
    pub position: [i32; 2],
    pub texture: Texture<'a>,
    pub canvas: &'a mut Canvas<sdl2::video::Window>,
}