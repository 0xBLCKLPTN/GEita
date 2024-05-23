use sdl2::render::Canvas;

use super::line_2d::Line2D;
use super::GEitaComponentsEnum;

pub struct CoordinateLines2D<'a> {
    pub canvas: &'a mut Canvas<sdl2::video::Window>,
    pub child_components: Vec<GEitaComponentsEnum>,
}

impl<'a> CoordinateLines2D<'a> {
    pub fn draw(canvas: &'a mut Canvas<sdl2::video::Window>) -> Self {
        let mut child_components: Vec<GEitaComponentsEnum> = Vec::new();
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

        Self {
            canvas,
            child_components,
        }
    }
    pub fn add_component(&mut self, component: GEitaComponentsEnum) {
        self.child_components.push(component)
    }
}
