pub mod cube;
pub mod image;
pub mod rect_2d;

pub use image::ImageComp;
pub use rect_2d::Rect2D;

pub enum GeitaComponents {
    ImageComp,
    Rect2D,
}


pub trait ComponentSystem {
    fn add_component()
}