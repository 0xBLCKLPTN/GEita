extern crate sdl2;
extern crate imgui;
extern crate imgui_sdl2;
extern crate gl;
extern crate imgui_opengl_renderer;

use std::time::Instant;
use imgui::Condition;
use imgui::FontSource;
use imgui::FontGlyphRanges;
use imgui::Style;
use imgui::StyleVar;
mod geita_ui;
use geita_ui::{GeitaUi, *};
use imgui::Context;
//mod project_manager;
//use crate::project_manager::GeitaUi;

pub struct WindowSize {
  pub w: u32,
  pub h: u32,
}

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video = sdl_context.video().unwrap();
  let mut ws = WindowSize {w: 1000u32, h: 1000u32 };
  {
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);
  }
  

  let window = video.window("Geite Project Manager", ws.w, ws.h)
    .position_centered()
    .resizable()
    .opengl()
    .allow_highdpi()
    .build()
    .unwrap();

  let _gl_context = window.gl_create_context().expect("Couldn't create GL context");
  gl::load_with(|s| video.gl_get_proc_address(s) as _);

  let mut imgui = imgui::Context::create();
  imgui.set_ini_filename(None);
  //let style = Style::use_classic_colors();
  
  let glyph_range = FontGlyphRanges::cyrillic();
  let mono = imgui.fonts().add_font(&[FontSource::TtfData{
        data: include_bytes!("/Users/twofacedjanus/Documents/geita/assets/JB.ttf"), //this files are valid
        size_pixels: 16.0,
        config: Some(imgui::FontConfig {
          glyph_ranges: glyph_range,
          size_pixels: 16.0,
          ..Default::default()
        }),
    }]);


  let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);

  let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

  let mut event_pump = sdl_context.event_pump().unwrap();

  let mut last_frame = Instant::now();
  init_style(&mut imgui);
  
  'running: loop {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;

    for event in event_pump.poll_iter() {
      imgui_sdl2.handle_event(&mut imgui, &event);
      if imgui_sdl2.ignore_event(&event) { continue; }

      match event {
        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'running
        },
        _ => {}
      }
    }


    imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());

    let now = Instant::now();
    let delta = now - last_frame;
    let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
    last_frame = now;
    imgui.io_mut().delta_time = delta_s;
    let ui = imgui.frame();
    
    ui.show_demo_window(&mut true);
    ui.show_project_manager_window(&mut true);
    
    unsafe {
      gl::ClearColor(0.44, 0.44, 0.64, 0.5);
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    imgui_sdl2.prepare_render(&ui, &window);
    renderer.render(&mut imgui);

    window.gl_swap_window();
    println!("{:?}", window.size());
    ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
  }
}

