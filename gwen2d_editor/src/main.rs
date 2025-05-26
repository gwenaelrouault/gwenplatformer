use crate::gui::menu::Menu;
use eframe::egui::Ui;
use eframe::egui::ViewportBuilder;
use std::sync::Arc;
use tokio::runtime::Runtime;

mod gui {
    pub mod menu;
}

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
    menu: Menu,
}

impl GwenEditor {
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
            menu: Menu::new(),
        }
    }
}

impl eframe::App for GwenEditor {
    // EGUI frame update method
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.menu.show(ui, ctx);
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
