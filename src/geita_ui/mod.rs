use imgui::{
    Ui, Context, Condition,
    TabBar, TabItem,
    FontConfig, FontSource, FontGlyphRanges,
};
use crate::imgui;


pub trait GeitaUi {
    fn show_project_manager_window(&self, is_open: &bool);
}


pub fn init_style(imgui: &mut Context) {
    let mut style = imgui.style_mut();
    style.window_rounding = 11.5;
    style.window_border_size = 1.0;
    style.window_padding = [8.0, 8.0];
    style.frame_padding = [4.0, 4.0];
    style.item_spacing = [8.0, 4.0];
    style.use_classic_colors();
}


pub fn init_font(imgui: &mut Context) {
    // FIXME: https://github.com/imgui-rs/imgui-rs/issues/421
    let glyph_range = FontGlyphRanges::cyrillic();

    // FIXME: change folders to relative.
    let _ = imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("/Users/twofacedjanus/Documents/git/GEita/resources/fonts/JetBrainsMono.ttf"),
        size_pixels: 16.0,
        config: Some(FontConfig {
            glyph_ranges: glyph_range,
            size_pixels: 16.0,
            ..Default::default()
        }),
    }]);
}


impl GeitaUi for Ui {
    fn show_project_manager_window(&self, is_open: &bool) {
        let mut selected_tab = 0;
        self.window("Project Manager")
            .title_bar(false)
            .movable(true)
            .position([20.0, 20.0], Condition::FirstUseEver)
            .size([960.0, 659.0], Condition::FirstUseEver)
            .build(|| {
                TabBar::new("MyTabBar")
                    .build(&self, || {
                        TabItem::new("Локальные проекты")
                            .build(&self, || {
                                self.text("Local Projects");
                            });
                        TabItem::new("Удаленные проекты")
                            .build(&self, || {
                                self.text("Remote Projects");
                            });
                    });           
            });
    }
} 