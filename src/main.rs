extern crate imgui;
extern crate imgui_sdl2;
extern crate gl;
extern crate imgui_opengl_renderer;

mod geita_ui;
mod geita_core;
mod geita_components;

use std::path::Path;
use std::time::{Duration, Instant};

use imgui::sys::ImVec2;
use imgui::{ Condition, FontSource, FontGlyphRanges,
            Style, StyleVar, Context };

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};
use sdl2::{Sdl, VideoSubsystem};
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::pixels::Color;
use geita_core::window::CoreWindow;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureCreator;
use sdl2::render::Texture;
use sdl2::surface::Surface;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::render::TextureQuery;
use geita_components::{Rect2D, ImageComp};
use sdl2::ttf::Font;

fn load_font<'a>(path: &str, size: u16, font_context: &'a sdl2::ttf::Sdl2TtfContext) -> Font<'a, 'static> {
    let ttf_context = font_context;
    let font = ttf_context.load_font(path, size).unwrap();
    font
}

fn create_texture_from_text<'a>(text: &str, font: &Font<'a, 'static>, texture_creator: &'a TextureCreator<sdl2::video::WindowContext>) -> Texture<'a> {
    let surface = font.render(text)
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface)
        .unwrap();

    texture
}

fn render_text<'a>(canvas: &mut Canvas<Window>, text: &str, font: &Font<'a, 'static>, texture_creator: &TextureCreator<sdl2::video::WindowContext>, x: i32, y: i32) {
    let texture = create_texture_from_text(text, font, texture_creator);

    let TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(x, y, width, height);

    canvas.copy(&texture, None, Some(target)).unwrap();
}

fn main() {
    let mut window = CoreWindow::new("Project Manager".to_string());
    let mut canvas = window.window.into_canvas().build().unwrap();
    let mut event_pump = window.sdl_context.event_pump().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let texture_creator = canvas.texture_creator();
    let font = load_font("/mnt/c/Users/oksan/OneDrive/Documents/GitHub/geita_project/Fortnight-resources/ttf/JetBrainsMono-Medium.ttf", 36, &ttf_context);
   
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        
        Rect2D::draw(&mut canvas, &mut [300i32,300i32], &texture_creator, &mut [256 as usize; 2]);
        
        let logo = Path::new("/mnt/c/Users/oksan/OneDrive/Documents/GitHub/geita_project/Fortnight-resources/Assets/hollow_knight_LARGE_ICO.png");
        let player = Path::new("/mnt/c/Users/oksan/OneDrive/Documents/GitHub/geita_project/Fortnight-resources/Assets/mario2d.png");
        render_text(&mut canvas, "Привет, SDL2!", &font, &texture_creator, 100, 100);
        //ImageComp::draw(&mut canvas, &mut [700i32,700i32], &texture_creator, logo, &mut [100u32, 100u32]);
        ImageComp::draw(&mut canvas, &mut [100i32,700i32], &texture_creator, player, &mut [100u32, 100u32]);

        canvas.present();
    }
}