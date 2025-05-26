use egui::Context;

pub trait Gui {
    fn ui(&mut self, ctx: &Context);
}
