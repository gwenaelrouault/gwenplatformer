use egui::Context;
use g2d_engine::gui::gui::Gui;

pub struct GameGui {
    
}

impl GameGui {
    pub fn new() -> Self {
        Self {}
    }
}

impl Gui for GameGui {
    fn ui(&mut self, ctx: &Context) {

    }
}