use crate::db::engine_db;
use crate::gui::tab_project::TabProject;
use eframe::egui;
use eframe::egui::ViewportBuilder;
use egui::{TextBuffer, Widget};
use gui::tab_entities::TabEntities;
use log::info;
use model::project::Project;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

mod db {
    pub mod engine_db;
}

mod model {
    pub mod entity;
    pub mod entity_category;
    pub mod entity_state;
    pub mod project;
}

mod gui {
    pub mod tab_entities;
    pub mod tab_project;
}

#[derive(PartialEq, Debug)]
enum EngineEditorTab {
    Project,
    Character,
    Scene,
    Level,
}

fn main() -> eframe::Result {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("START");
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1024., 768.]),
        ..Default::default()
    };
    eframe::run_native(
        "GWEN 2D ENGINE",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    name: String,
    runtime: Arc<Runtime>,
    age: u32,
    tab: EngineEditorTab,
    tab_entities: TabEntities,
    tab_project: TabProject,
    window_open: bool,
    name_character: String,
    project: Project,
    db: Arc<Mutex<engine_db::EngineDb>>,
}

impl MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            runtime: Arc::new(Runtime::new().unwrap()),
            age: 42,
            tab: EngineEditorTab::Project,
            tab_entities: TabEntities::new(),
            tab_project: TabProject::new(),
            window_open: false,
            name_character: String::new(),
            project: Project::new("default".to_string()),
            db: Arc::new(Mutex::new(engine_db::EngineDb::new())),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, EngineEditorTab::Project, "Projet");
                if self.tab_project.is_loaded() {
                    ui.selectable_value(&mut self.tab, EngineEditorTab::Character, "Personnages");
                    ui.selectable_value(&mut self.tab, EngineEditorTab::Scene, "Scenes");
                    ui.selectable_value(&mut self.tab, EngineEditorTab::Level, "Niveaux");
                }
            });
            ui.separator();

            match self.tab {
                EngineEditorTab::Character => {
                    self.tab_entities
                        .show_ui(ui, ctx, &mut self.project, &mut self.db);
                }
                EngineEditorTab::Scene => {
                    ui.group(|ui| {
                        ui.set_min_height(ctx.screen_rect().height() - 40.);
                        ui.set_min_width(ctx.screen_rect().width() - 26.);
                    });
                }
                EngineEditorTab::Level => {
                    ui.group(|ui| {
                        ui.set_min_height(ctx.screen_rect().height() - 40.);
                        ui.set_min_width(ctx.screen_rect().width() - 26.);
                    });
                }
                _ => {
                    self.tab_project
                        .show_ui(ui, ctx, &mut self.runtime, &mut self.db);
                }
            }

            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
