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
//pub mod scripts;

use geita_core::types::BoxedResult;

use geita_components::camera_2d::Camera2D;
use geita_components::rect_2d::Rect2D;
use geita_components::image_2d::Image2D;
use geita_components::{GEitaComponentSystem, GEitaComponentsEnum};
use geita_core::window::{handle_event, GeitaWindow};

use std::time::Duration;
use std::path::Path;

use sdl2::mouse::Cursor;
use sdl2::surface::{ Surface};
use sdl2::image::LoadSurface;
//use scripts::terrain_generator::TerrainGenerator2D;



pub(crate) fn main() -> BoxedResult<()> {
    let mut window = GeitaWindow::init("object", [800u32, 600u32])?;
    let mut camera = Camera2D::new();
    let mut component_system = GEitaComponentSystem::init();
        
    //let cursor_surface = Surface::from_ll("~/Desktop/GEita/resources/textures/cursor.png");
    //let cursor_texture = texture_creator.create_texture_from_surface(&cursor_surface)?;
    //let cursor = Cursor::from_surface(cursor_surface, 16i32, 16i32)?;
    //cursor_surface.free();
    //cursor.set();
    
    //let cursor_rect = Rect::new(0,0, cursor_texture.query().width, cursor_texture.query().height);
    //window.canvas.set_cursor(&cursor_texture, cursor_rect)?;        
    
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

        
        
        window.canvas.present();
    }
    Ok(())
}
