use std::sync::Arc;
use tokio::runtime::Runtime;
use log::info;
use eframe::egui::ViewportBuilder;
use eframe::egui::{Align2, ColorImage, Pos2, TextureHandle, Ui, Vec2};

#[derive(PartialEq, Debug)]
enum EngineEditorTab {
    Project,
    Character,
    Scene,
    Level,
}

struct GwenEditor {
        runtime: Arc<Runtime>,
        tab: EngineEditorTab,
}

impl GwenEditor {
    fn show_menu(&mut self, ui: &mut Ui, ctx: &egui::Context) {
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

    fn show_tabs(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.tab, EngineEditorTab::Project, "Explorer");
            ui.selectable_value(&mut self.tab, EngineEditorTab::Character, "Entities");
            ui.selectable_value(&mut self.tab, EngineEditorTab::Scene, "Scenes");
            ui.selectable_value(&mut self.tab, EngineEditorTab::Level, "Levels");
        });
    }
}

impl Default for GwenEditor {
    fn default() -> Self {
        Self {
            runtime: Arc::new(Runtime::new().unwrap()),
            tab: EngineEditorTab::Project,
        }
    }
}

impl eframe::App for GwenEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.show_menu(ui, ctx);
            self.show_tabs(ui, ctx);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Bienvenue dans l'application !");
        });
    }

}

fn main() -> eframe::Result {
    // configure logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // configure viewport
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1024., 768.]),
        ..Default::default()
    };

    // show app
    eframe::run_native(
        "GWEN 2D EDITOR",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<GwenEditor>::default())
        }),
    )
}
