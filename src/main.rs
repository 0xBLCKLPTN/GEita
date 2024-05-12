extern crate imgui;
extern crate imgui_sdl2;
extern crate gl;
extern crate imgui_opengl_renderer;

mod geita_ui;
mod geita_core;

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

pub struct Entity2D<'a> {
    pub texture: Texture<'a>,
    pub size: ImVec2,
    pub position: ImVec2,
}

fn draw_2d_cube(canvas: &mut Canvas<sdl2::video::Window>, position: &mut [i32; 2]) {
    let texture_creator = canvas.texture_creator();
    let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 512, 512 ).unwrap();
    
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in (0..256) {
            for x in (0..256) {
                let offset = y*pitch + x*3;
                buffer[offset + 0] = x as u8;
                buffer[offset + 1] = y as u8;
                buffer[offset + 2] = 0;
            }
        }
    }).unwrap();

    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    let rect1 = Rect::new(position[0], position[1], 250, 250);
    canvas.copy(&texture, None, rect1).unwrap();
}


fn main() {
    let mut window = CoreWindow::new("Project Manager".to_string());
    let mut canvas = window.window.into_canvas().build().unwrap();
    let mut event_pump = window.sdl_context.event_pump().unwrap();
    //let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, (256, 256)).unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let texture_creator = canvas.texture_creator();
    let png = Path::new("/Users/twofacedjanus/Documents/geita_project/Fortnight-resources/Assets/hollow_knight_LARGE_ICO.png");
    let texture = texture_creator.load_texture(png).unwrap();

    canvas.copy(&texture, None, None);
    canvas.present();
    
    
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

        draw_2d_cube(&mut canvas, &mut [300i32,300i32]);
        draw_2d_cube(&mut canvas, &mut [700i32,700i32]);

        canvas.present();
    }
}