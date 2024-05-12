use std::path::Path;

pub struct Image {
    pub image_path: Path,
    pub canvas: &mut Canvas<WindowContext>,
    pub position: &mut [i32; 2],
}

impl Image {
    pub fn draw(canvas: &mut Canvas<sdl2::video::Window>, position: &mut [i32; 2], texture_creator: &TextureCreator<WindowContext>) -> Self {
        let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
        let png = Path::new("/mnt/c/Users/oksan/OneDrive/Documents/GitHub/geita_project/Fortnight-resources/Assets/hollow_knight_LARGE_ICO.png");
        let texture = texture_creator.load_texture(png).unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        let rect1 = Rect::new(position[0], position[1], 250, 250);
        canvas.copy(&texture, None, rect1).unwrap();
        Self { png, }
    }
}