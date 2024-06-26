use crate::imgui;
use crate::imgui_opengl_renderer::Renderer;
use imgui::{Condition, Context, FontConfig, FontGlyphRanges, FontSource, TabBar, TabItem, Ui};
use imgui_sdl2::ImguiSdl2;
use sdl2::image::{InitFlag, LoadTexture, Sdl2ImageContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;
use sdl2::{Sdl, VideoSubsystem};

enum KKK {
    Ranvas(Canvas<sdl2::video::Window>),
    Wanwas(Window),
}

pub struct CoreWindow {
    pub window_height: u32,
    pub window_width: u32,
    pub window_title: String,
    pub sdl_context: Sdl,
    pub video: VideoSubsystem,
    pub canvas: Canvas<sdl2::video::Window>,
    pub ttf_context: Sdl2TtfContext,
    pub image_context: Sdl2ImageContext,
    pub texture_creator: TextureCreator<WindowContext>, //pub event_pump: EventPump,
    pub imgui: Context,
    pub imgui_sdl2: ImguiSdl2,
    pub renderer: Renderer,
    pub window: Option<Window>,
}

impl CoreWindow {
    pub fn new(title: String) -> CoreWindow {
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();
        let mut window_width = 1000u32;
        let mut window_height = 1000u32;
        let mut event_pump = sdl_context.event_pump().unwrap();

        let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();

        {
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 0);
        }

        let mut window = video
            .window(&title, window_height, window_width)
            .position_centered()
            .resizable()
            .opengl()
            .allow_highdpi()
            .build()
            .unwrap();

        let _gl_context = window
            .gl_create_context()
            .expect("Couldn't create GL context");
        gl::load_with(|s| video.gl_get_proc_address(s) as _);

        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);

        let renderer =
            imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        return CoreWindow {
            window: None,
            renderer,
            imgui_sdl2,
            sdl_context,
            video,
            canvas,
            imgui,
            //event_pump,
            window_width,
            window_height,
            ttf_context,
            image_context,
            window_title: title.clone(),
            texture_creator,
        };
    }
}
