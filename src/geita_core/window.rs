use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};

pub struct CoreWindow {
    pub window_height: u32,
    pub window_width: u32,
    pub window_title: String,
    pub sdl_context: Sdl,
    pub video: VideoSubsystem,
    pub window: Window,
}

impl CoreWindow {
    pub fn new(title: String) -> CoreWindow {
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();
        let mut window_width = 1000u32;
        let mut window_height = 1000u32;

        {
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 0);
        }

        let window = video.window(&title, window_height, window_width)
            .position_centered()
            .resizable()
            .opengl()
            .allow_highdpi()
            .build()
            .unwrap();

        let _gl_context = window.gl_create_context().expect("Couldn't create GL context");
        gl::load_with(|s| video.gl_get_proc_address(s) as _);
        
        return CoreWindow {
            sdl_context,
            video,
            window,
            window_width,
            window_height,
            window_title: title.clone()

        }
    }

}