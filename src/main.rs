extern crate imgui;
extern crate imgui_sdl2;
extern crate gl;
extern crate imgui_opengl_renderer;

mod geita_ui;

use std::time::Instant;

use imgui::{ Condition, FontSource, FontGlyphRanges,
            Style, StyleVar, Context };

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};

pub struct WindowConfig {
    width: u32,
    height: u32,
    window_name: String,
}

pub struct ImguiFeatures {
    imgui_initialized: i8,
}

pub struct CoreWindow {
    sdl_context: Sdl,
    video: VideoSubsystem,
    window_config: WindowConfig,
    imgui_features: ImguiFeatures,
    window: Window,
}

impl CoreWindow {
    pub fn new(window_name: String) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();

        let mut width = 1000u32;
        let mut height = 1000u32;
        let mut wc = WindowConfig { width, height, window_name: window_name.clone() };

        {
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 0);
        }

        let window = video.window(&window_name, wc.height, wc.height)
            .position_centered()
            .resizable()
            .opengl()
            .allow_highdpi()
            .build()
            .unwrap();

        let _gl_context = window.gl_create_context().expect("Couldn't create GL context");
        gl::load_with(|s| video.gl_get_proc_address(s) as _);

        let mut imgui_features = ImguiFeatures::new(1i8);

        return CoreWindow {
            sdl_context,
            video,
            window_config: wc,
            imgui_features,
        }
    }

    
    /*
    pub fn init_imgui(&mut self) {
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
    }
    */
}

impl ImguiFeatures {
    pub fn new(incl: i8, window: &Window) -> Self {
        return match incl {
            0i8 => { Self { imgui_initialized: incl } },
            1i8 => {
                let mut imgui = imgui::Context::create();
                let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&imgui, &window);
                let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s|, video.gl_get_proc_address(s) as _);
                Self { imgui_initialized: incl, imgui, imgui_sdl2, renderer } },
            _ => { Self { imgui_initialized: 0i8 } },
        };
    }
}


fn main() {    
    //let mut core
    println!("");
}
