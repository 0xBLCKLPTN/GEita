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

use geita_components::{text, Camera, CoordinateLines2D, ImageComp, Rect2D};
use geita_core::window::CoreWindow;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator, TextureQuery};
use sdl2::surface::Surface;
use sdl2::sys::KeyCode;
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use sdl2::{Sdl, VideoSubsystem};

fn project(coord: [f32; 3], focal_length: f32, width: f32, height: f32) -> (i32, i32) {
    let x = (coord[0] * focal_length) / coord[2] + (width / 2.0);
    let y = (coord[1] * focal_length) / coord[2] + (height / 2.0);
    (x.round() as i32, y.round() as i32)
}

fn main() {
    let mut window = CoreWindow::new("Project Manager".to_string());
    let mut event_pump = window.sdl_context.event_pump().unwrap();

    let mut camera = Camera::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            /*
            window.imgui_sdl2.handle_event(&mut window.imgui, &event);
            if window.imgui_sdl2.ignore_event(&event) {
                continue;
            }
            */

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    camera.position[1] -= 10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    camera.position[1] += 10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    camera.position[0] -= 10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    camera.position[0] += 10;
                }
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        window
            .canvas
            .set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        window.canvas.clear();
        CoordinateLines2D::draw(&mut window.canvas);
        /*
        let ui = window.imgui.frame();
        ui.show_demo_window(&mut true);

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        //window.window = Some(window.canvas.into_window());

        window
            .imgui_sdl2
            .prepare_render(&ui, &window.window.unwrap());
        window.renderer.render(&mut window.imgui);

        window.window.unwrap().gl_swap_window();
        */

        Text::draw(
            "/Users/twofacedjanus/Documents/geita_project/Fortnight-resources/Fonts/JBMono/ttf/JetBrainsMono-Medium.ttf",
            36,
            &window.ttf_context,
            &window.texture_creator,
            "Some Text Object",
            &mut [100i32; 2],
            &mut window.canvas,
            &mut camera
        );

        //let logo = Path::new("/mnt/c/Users/oksan/OneDrive/Documents/GitHub/geita_project/Fortnight-resources/Assets/hollow_knight_LARGE_ICO.png");
        //let player = Path::new("/mnt/c/Users/oksan/OneDrive/Documents/GitHub/geita_project/Fortnight-resources/Assets/mario2d.png");
        let logo = Path::new("/Users/twofacedjanus/Documents/geita_project/Fortnight-resources/Assets/hollow_knight_LARGE_ICO.png");
        let player = Path::new(
            "/Users/twofacedjanus/Documents/geita_project/Fortnight-resources/Assets/hollow_knight_player.png",
        );

        //ImageComp::draw(&mut canvas, &mut [700i32,700i32], &texture_creator, logo, &mut [100u32, 100u32]);
        ImageComp::draw(
            &mut window.canvas,
            &mut [100i32, 700i32],
            &window.texture_creator,
            player,
            &mut [200u32, 200u32],
            &mut camera,
        );

        window.canvas.present();
    }
}
