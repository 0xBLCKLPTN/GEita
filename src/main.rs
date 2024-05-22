/*
    Starting point of the ita game engine.
    Author: Two Faced Janus.
*/

extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;

pub mod geita_components;
pub mod geita_core;

use geita_core::types::BoxedResult;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use geita_components::camera_2d::Camera2D;
use geita_core::window::GeitaWindow;

use std::time::Duration;

fn main() -> BoxedResult<()> {
    let mut window = GeitaWindow::init("object", [800u32, 600u32])?;
    let mut camera = Camera2D::new();

    'running: loop {
        for event in window.event_pump.poll_iter() {
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
        window.canvas.present();
    }
    Ok(())
}
