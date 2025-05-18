use crate::db::engine_db::EngineDb;
use dirs_next::home_dir;
use eframe::egui::{Align2, Pos2, Ui, Vec2};
use eframe::epaint::FontId;
use egui::{Color32, RichText};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

pub struct TabProject {
    path: Option<String>,
    new_project_name: String,
    new_project_path: String,
    project_creation_window: bool,
    loaded_project: bool,
}

impl TabProject {
    pub fn new() -> Self {
        TabProject {
            path: None,
            new_project_name: String::new(),
            new_project_path: home_dir().unwrap().to_string_lossy().to_string(),
            project_creation_window: false,
            loaded_project: false,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded_project
    }

    pub fn show_ui(
        &mut self,
        ui: &mut Ui,
        ctx: &egui::Context,
        runtime: &mut Arc<Runtime>,
        db: &mut Arc<Mutex<EngineDb>>,
    ) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                if ui
                    .button(RichText::new("Nouveau projet").font(FontId::proportional(20.0)))
                    .clicked()
                {
                    self.project_creation_window = true;
                }
                ui.add_space(10.);
                if ui
                    .button(RichText::new("Ouvrir un projet").font(FontId::proportional(20.0)))
                    .clicked()
                {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.path = Some(path.display().to_string());
                    }
                }
                if self.manage_project_creation(ui, ctx, runtime, db) {
                    self.project_creation_window = false;
                }
            });
            ui.add_space(100.);
            let text = if self.loaded_project {
                RichText::new(self.new_project_name.clone())
            } else {
                RichText::new("Aucun projet chargé")
            };
            ui.label(text.font(FontId::proportional(40.0)).color(Color32::WHITE))
        });
        self.display_stats(ui, ctx);
    }

    fn manage_project_creation(
        &mut self,
        ui: &mut Ui,
        ctx: &eframe::egui::Context,
        runtime: &mut Arc<Runtime>,
        db: &mut Arc<Mutex<EngineDb>>,
    ) -> bool {
        let current_pos = Pos2::new(100., 100.);
        let mut cancel_window = false;
        egui::Window::new("Nouveau projet")
            .collapsible(false)
            .resizable(false)
            .current_pos(current_pos)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0., 0.))
            .open(&mut self.project_creation_window)
            .show(ctx, |ui| {
                let mut db_path = None;
                let mut root_path = PathBuf::from_str(&self.new_project_path).unwrap();
                ui.horizontal(|ui| {
                    ui.label("Nom du projet");
                    ui.text_edit_singleline(&mut self.new_project_name);
                });
                if !self.new_project_name.is_empty() {
                    db_path = Some(root_path.join(self.new_project_name.clone()));
                }
                if ui.button("Répertoire").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        db_path = Some(path.join(self.new_project_name.clone()));
                    }
                }
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    let save_button = egui::Button::new("Sauvegarder");
                    match db_path {
                        Some(path) => {
                            ui.add_enabled(true, save_button).clicked().then(|| {
                                create_new_project(runtime, db, &path.clone());
                                self.loaded_project = true;
                                cancel_window = true
                            });
                        }
                        None => {
                            ui.add_enabled(false, save_button);
                        }
                    }
                    ui.button("Quitter").clicked().then(|| cancel_window = true)
                });
                ui.label(self.new_project_path.clone());
            });
        cancel_window
    }

    fn display_stats(&mut self, ui: &mut Ui, ctx: &eframe::egui::Context) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Nombre de niveaux");
            });
            ui.horizontal(|ui| {
                ui.label("Nombre d'entités'");
            });
        });
    }
}

fn create_new_project(
    runtime: &mut Arc<Runtime>,
    db: &mut Arc<Mutex<EngineDb>>,
    db_path: &PathBuf,
) {
    let mut db_to_open = db.clone();
    let path = db_path.clone();
    runtime.spawn(async move {
        let mut db_locked = db_to_open.lock().await;
        db_locked.open(&path).await;
    });
}
