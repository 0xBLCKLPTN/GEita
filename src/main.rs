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
use geita_components::{Rect2D, ImageComp};


fn main() {
    let mut window = CoreWindow::new("Project Manager".to_string());
    let mut canvas = window.window.into_canvas().build().unwrap();
    let mut event_pump = window.sdl_context.event_pump().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let texture_creator = canvas.texture_creator();
   
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

        Rect2D::draw(&mut canvas, &mut [300i32,300i32], &texture_creator);
        let logo = Path::new("/mnt/c/Users/oksan/OneDrive/Documents/GitHub/geita_project/Fortnight-resources/Assets/hollow_knight_LARGE_ICO.png");
        let player = Path::new("/mnt/c/Users/oksan/OneDrive/Documents/GitHub/geita_project/Fortnight-resources/Assets/hollow_knight_player.png");
        
        ImageComp::draw(&mut canvas, &mut [700i32,700i32], &texture_creator, logo);
        ImageComp::draw(&mut canvas, &mut [100i32,700i32], &texture_creator, player);

        canvas.present();
    }
}