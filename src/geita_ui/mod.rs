use imgui::{Ui, Condition, TabBar};
use imgui::{FontConfig, FontSource};
use crate::imgui;
use imgui::Context;
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
                        imgui::TabItem::new("Локальные проекты")
                            .build(&self, || {
                                self.text("Local Projects");
                            });
                        imgui::TabItem::new("Удаленные проекты")
                            .build(&self, || {
                                self.text("Remote Projects");
                            });
                    });           
            });
    }
} 
