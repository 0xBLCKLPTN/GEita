/*
    Сreating and managing a window.
    Autor: Two Faced Janus.
*/

use super::types::BoxedResult;
use crate::imgui;
use crate::imgui_opengl_renderer::Renderer;
use imgui::Context;
use imgui_sdl2::ImguiSdl2;
use sdl2::image::{InitFlag, Sdl2ImageContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;
use sdl2::EventPump;
use sdl2::{Sdl, VideoSubsystem};

pub struct WindowConfig<'a> {
    pub sizes: [u32; 2],
    pub title: &'a str,
    pub gui: Option<WindowGui>,
}

pub struct WindowGui {
    imgui: Context,
    imgui_sdl2: ImguiSdl2,
}

pub struct GeitaWindow<'a> {
    pub window_config: WindowConfig<'a>,
    pub sdl_context: Sdl,
    pub video: VideoSubsystem,
    pub canvas: Canvas<sdl2::video::Window>,
    pub ttf_context: Sdl2TtfContext,
    pub image_context: Sdl2ImageContext,
    pub texture_creator: TextureCreator<WindowContext>,
    pub renderer: Renderer,
    pub event_pump: EventPump,
}

impl<'a> GeitaWindow<'a> {
    pub fn init(window_title: &str, default_sizes: [u32; 2]) -> BoxedResult<GeitaWindow> {
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;

        let window_config = WindowConfig {
            sizes: default_sizes,
            title: window_title,
            gui: None,
        };

        let mut event_pump = sdl_context.event_pump()?;

        let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
        let ttf_context = sdl2::ttf::init()?;

        // Init GL attributes
        {
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 0);
        }

        let mut window = video
            .window(&window_title, default_sizes[0], default_sizes[1])
            .position_centered()
            .resizable()
            .opengl()
            .allow_highdpi()
            .build()?;

        let _gl_context = window
            .gl_create_context()
            .expect("Couldn't create GL context");

        gl::load_with(|s| video.gl_get_proc_address(s) as _);

        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);

        let renderer =
            imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

        let mut canvas = window.into_canvas().build()?;
        let texture_creator = canvas.texture_creator();

        Ok(GeitaWindow {
            window_config,
            canvas,
            video,
            ttf_context,
            image_context,
            sdl_context,
            texture_creator,
            renderer,
            event_pump,
        })
    }
}
