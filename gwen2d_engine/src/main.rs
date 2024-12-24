use egui_modal::Modal;
use gui::tab_entities::TabEntities;
use model::project::Project;
use std::fmt::Debug;
use egui::{Align2, ComboBox, Pos2, Response, TextBuffer, Vec2, ViewportBuilder, Widget};
use eframe::egui;
mod model {
    pub mod entity_category;
    pub mod entity;
    pub mod entity_state;
    pub mod project;
}

mod gui  {
    pub mod tab_entities;
}

#[derive(PartialEq, Debug)]
enum EngineEditorTab {
    Project,
    Character,
    Scene,
    Level,
}


fn main() -> eframe::Result {
    env_logger::init();
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
    age: u32,
    tab: EngineEditorTab,
    tab_entities: TabEntities,
    window_open : bool,
    name_character : String,
    project : model::project::Project,
}

impl MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            tab: EngineEditorTab::Character,
            tab_entities: TabEntities::new(),
            window_open:false,
            name_character : String::new(),
            project : Project::new("default".to_string())
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, EngineEditorTab::Project, "Projet");
                ui.selectable_value(&mut self.tab, EngineEditorTab::Character, "Personnages");
                ui.selectable_value(&mut self.tab, EngineEditorTab::Scene, "Scenes");
                ui.selectable_value(&mut self.tab, EngineEditorTab::Level, "Niveaux");
            });
            ui.separator();

            match self.tab {
                EngineEditorTab::Character => {
                    self.tab_entities.show_ui(ui, ctx, &mut self.project);
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
                    ui.group(|ui| {
                        ui.label("Within a frame");
                        ui.set_min_height(ctx.screen_rect().height() - 40.);
                        ui.set_min_width(ctx.screen_rect().width() - 26.);
                    });
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

