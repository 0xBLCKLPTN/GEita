extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;

mod geita_components;
mod geita_core;
mod geita_ui;

use std::path::Path;
use std::time::{Duration, Instant};

use geita_components::text::Text;
use imgui::sys::ImVec2;
use imgui::{Condition, Context, FontGlyphRanges, FontSource, Style, StyleVar};

use geita_components::{text, ImageComp, Rect2D};
use geita_core::window::CoreWindow;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator, TextureQuery};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use sdl2::{Sdl, VideoSubsystem};

fn main() {
    let mut window = CoreWindow::new("Project Manager".to_string());
    let mut canvas = window.window.into_canvas().build().unwrap();
    let mut event_pump = window.sdl_context.event_pump().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let texture_creator = canvas.texture_creator();
    //let font = text::load_font("/Users/twofacedjanus/Documents/geita_project/Fortnight-resources/Fonts/JBMono/ttf/JetBrainsMono-Medium.ttf", 36, &ttf_context);

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

        Rect2D::draw(
            &mut canvas,
            &mut [300i32, 300i32],
            &texture_creator,
            &mut [256 as usize; 2],
        );

        Text::draw(
            "/Users/twofacedjanus/Documents/geita_project/Fortnight-resources/Fonts/JBMono/ttf/JetBrainsMono-Medium.ttf",
            36,
            &ttf_context,
            &texture_creator,
            "Welcome to SDL2",
            &mut [100i32; 2],
            &mut canvas);

        //let logo = Path::new("/mnt/c/Users/oksan/OneDrive/Documents/GitHub/geita_project/Fortnight-resources/Assets/hollow_knight_LARGE_ICO.png");
        //let player = Path::new("/mnt/c/Users/oksan/OneDrive/Documents/GitHub/geita_project/Fortnight-resources/Assets/mario2d.png");
        let logo = Path::new("/Users/twofacedjanus/Documents/geita_project/Fortnight-resources/Assets/hollow_knight_LARGE_ICO.png");
        let player = Path::new(
            "/Users/twofacedjanus/Documents/geita_project/Fortnight-resources/Assets/hollow_knight_player.png",
        );

        //ImageComp::draw(&mut canvas, &mut [700i32,700i32], &texture_creator, logo, &mut [100u32, 100u32]);
        ImageComp::draw(
            &mut canvas,
            &mut [100i32, 700i32],
            &texture_creator,
            player,
            &mut [100u32, 100u32],
        );

        canvas.present();
    }
}
