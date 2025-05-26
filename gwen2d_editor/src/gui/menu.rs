pub struct Menu {}

impl Menu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut eframe::egui::Ui, ctx: &egui::Context) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Project", |ui| {
                if ui.button("New project").clicked() {
                    ui.close_menu();
                }
                if ui.button("Open project").clicked() {
                    ui.close_menu();
                }
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
        });
    }
}
