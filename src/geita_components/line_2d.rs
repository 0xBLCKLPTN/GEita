use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;

use super::GEitaComponentsEnum;

pub struct Line2D<'a> {
    pub a_pos: [i32; 2],
    pub b_pos: [i32; 2],
    pub color: [u8; 3],
    pub canvas: &'a mut Canvas<sdl2::video::Window>,
    pub child_components: Vec<GEitaComponentsEnum>,
}

impl<'a> Line2D<'a> {
    pub fn draw(
        canvas: &'a mut Canvas<sdl2::video::Window>,
        color: [u8; 3],
        a_pos: [i32; 2],
        b_pos: [i32; 2],
    ) -> Self {
        let mut child_components: Vec<GEitaComponentsEnum> = Vec::new();
        canvas.set_draw_color(Color::RGB(color[0], color[1], color[2]));
        canvas
            .draw_line(
                Point::new(a_pos[0], a_pos[1]),
                Point::new(b_pos[0], b_pos[1]),
            )
            .unwrap();

        Self {
            canvas,
            a_pos,
            b_pos,
            color,
            child_components,
        }
    }
    pub fn add_component(&mut self, component: GEitaComponentsEnum) {
        self.child_components.push(component)
    }
}
