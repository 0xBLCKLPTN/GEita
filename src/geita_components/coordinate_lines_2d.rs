use crate::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture, TextureCreator};

pub struct CoordinateLines2D<'a> {
    pub canvas: &'a mut Canvas<sdl2::video::Window>,
}

impl<'a> CoordinateLines2D<'a> {
    pub fn draw(canvas: &'a mut Canvas<sdl2::video::Window>) -> Self {
        // Coordinates
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas
            .draw_line(Point::new(500, 500), Point::new(1000, 500))
            .unwrap();
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas
            .draw_line(Point::new(750, 250), Point::new(750, 750))
            .unwrap();

        Self { canvas }
    }
}
