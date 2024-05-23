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

use geita_components::camera_2d::Camera2D;
use geita_components::rect_2d::Rect2D;

use geita_core::window::{handle_event, GeitaWindow};
use std::time::Duration;

pub(crate) fn main() -> BoxedResult<()> {
    let mut window = GeitaWindow::init("object", [800u32, 600u32])?;
    let mut camera = Camera2D::new();

    'running: loop {
        // FIMXE: move into another file - events.rs.
        for event in window.event_pump.poll_iter() {
            match handle_event(event, &mut camera) {
                Some(_) => break 'running,
                None => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        window
            .canvas
            .set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        window.canvas.clear();

        // <-------| Test

        Rect2D::draw(
            &mut window.canvas,
            &mut [300i32, 300i32],
            &window.texture_creator,
            &mut [256 as usize; 2],
        );

        // |------->

        window.canvas.present();
    }
    Ok(())
}
