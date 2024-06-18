pub mod camera_2d;
pub mod coordinates_2d;
pub mod image_2d;
pub mod line_2d;
pub mod rect_2d;
pub mod text_2d;

use camera_2d::Camera2D;
use coordinates_2d::CoordinateLines2D;
use image_2d::Image2D;
use line_2d::Line2D;
use rect_2d::Rect2D;
use text_2d::Text;


pub enum GEitaComponentsEnum {
    Camera2D,
    CoordinateLines2D,
    Line2D,
    Rect2D,
    Text,
    JopaNegra {
        a: i32,
        b: i32,
    },
    None
}




pub struct GEitaComponentSystem {
    pub child_components: Vec<GEitaComponentsEnum>,
}

impl GEitaComponentSystem {
    pub fn init() -> Self {
        let mut child_components: Vec<GEitaComponentsEnum> = Vec::new();
        Self { child_components }
    }

    pub fn add(&mut self, component: GEitaComponentsEnum) {
        self.child_components.push(component);
    }
}
