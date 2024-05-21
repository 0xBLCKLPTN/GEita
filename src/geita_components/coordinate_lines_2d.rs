use crate::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;

use super::Line2D;

pub struct CoordinateLines2D<'a> {
    pub canvas: &'a mut Canvas<sdl2::video::Window>,
}

impl<'a> CoordinateLines2D<'a> {
    pub fn draw(canvas: &'a mut Canvas<sdl2::video::Window>) -> Self {
        // Coordinates
        Line2D::draw(
            canvas,
            [255u8, 0u8, 0u8],
            [500i32, 500i32],
            [1000i32, 500i32],
        );
        Line2D::draw(
            canvas,
            [0u8, 255u8, 0u8],
            [750i32, 250i32],
            [750i32, 750i32],
        );

        Self { canvas }
    }
}
