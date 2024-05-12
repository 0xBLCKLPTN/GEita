pub mod cube;
pub mod image;
pub mod rect_2d;

pub use image::ImageComp;
pub use rect_2d::Rect2D;

pub enum GeitaComponents {
    ImageComp,
    Rect2D,
}

pub struct Scene2D {
    pub components: Vec<GeitaComponents>,
}

impl Scene2D {
    pub fn init() -> Scene2D {
        let mut components = Vec::new();
        Scene2D { components }
    }

    pub fn add_child(&mut self, component: GeitaComponents) {
        &self.components.push(component);
    }
}


pub trait ComponentSystem {
    fn add_child(&mut self, child: GeitaComponents);
}
