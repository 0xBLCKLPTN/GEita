pub mod camera;
pub mod coordinate_lines_2d;
pub mod cube;
pub mod image;
pub mod line;
pub mod rect_2d;
pub mod text;

pub use camera::Camera;
pub use coordinate_lines_2d::CoordinateLines2D;
pub use image::ImageComp;
pub use line::Line2D;
pub use rect_2d::Rect2D;
pub use text::Text;

pub enum GeitaComponents {
    ImageComp,
    Rect2D,
    Camera,
    Text,
}
