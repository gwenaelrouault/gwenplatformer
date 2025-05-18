use std::sync::Arc;
use crate::model::{
    entity::Entity, entity_category::EntityCategory, entity_state::EntityState, project::Project,
};
use eframe::egui;
use eframe::egui::{Align2, ColorImage, Pos2, TextureHandle, Ui, Vec2};
use image::{io::Reader as ImageReader, DynamicImage};
use tokio::sync::Mutex;
use crate::db::engine_db::EngineDb;

pub struct TabEntities {
    selected_category: EntityCategory,
    selected_entity: Entity,
    selected_state: EntityState,
    category_creation_window: bool,
    entity_creation_window: bool,
    state_creation_window: bool,
    name: String,
    new_category: String,
    new_entity: String,
    new_state: String,
    images: Vec<Option<TextureHandle>>,
    image_texture: Option<TextureHandle>,
}

impl TabEntities {
    pub fn new() -> Self {
        TabEntities {
            selected_category: EntityCategory::default(),
            selected_entity: Entity::default(EntityCategory::default()),
            selected_state: EntityState::default(),
            category_creation_window: false,
            entity_creation_window: false,
            state_creation_window: false,
            name: String::new(),
            new_category: String::new(),
            new_entity: String::new(),
            new_state: String::new(),
            images: vec![None; 5],
            image_texture: None,
        }
    }

    fn dynamic_image_to_color_image(image: &DynamicImage) -> Result<ColorImage, String> {
        let rgba_image = image.to_rgba8();
        let size = [rgba_image.width() as usize, rgba_image.height() as usize];
        let pixels = rgba_image.into_raw();
        Ok(ColorImage::from_rgba_unmultiplied(size, &pixels))
    }

    fn load_image_from_path(&mut self, path: &std::path::Path) -> Result<ColorImage, String> {
        let img = ImageReader::open(path)
            .map_err(|_| "Erreur lors de l'ouverture du fichier".to_string())?
            .decode()
            .map_err(|_| "Erreur lors du décodage de l'image".to_string())?;
        let img = img.to_rgba8();
        let size = [img.width() as usize, img.height() as usize];
        let pixels = img.into_raw();
        Ok(ColorImage::from_rgba_unmultiplied(size, &pixels))
    }

    pub fn show_ui(&mut self, ui: &mut Ui, ctx: &egui::Context, project: &mut Project, db: &mut Arc<Mutex<EngineDb>>,) {
        ui.group(|ui| {
            ui.set_min_height(ctx.screen_rect().height() - 40.);
            ui.set_min_width(ctx.screen_rect().width() - 26.);
            ui.horizontal(|ui| {
                ui.label("   Catégorie");
                egui::ComboBox::from_id_salt("Classe de l'entité")
                    .selected_text(format!("{:?}", self.selected_category.name()))
                    .show_ui(ui, |ui| {
                        for category_value in project.categories.borrow_mut().iter_mut() {
                            let label = category_value.name().clone();
                            ui.selectable_value(
                                &mut self.selected_category,
                                category_value.to_owned(),
                                format!("{:?}", label),
                            );
                        }
                    });
                if ui.button("+").clicked() {
                    self.category_creation_window = true;
                }
                if self.create_category(ui, ctx, project) {
                    self.category_creation_window = false;
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Entité");
                egui::ComboBox::from_id_salt("Nom de l'entité")
                    .selected_text(format!("{:?}", self.selected_entity.name()))
                    .show_ui(ui, |ui| {
                        for (k, v) in project.entities.borrow_mut().iter_mut() {
                            let label = v.name().clone();
                            ui.selectable_value(
                                &mut self.selected_entity,
                                v.to_owned(),
                                format!("{:?}", label),
                            );
                        }
                    });
                if ui.button("+").clicked() {
                    self.entity_creation_window = true;
                }
                if self.create_entity(ui, ctx, project) {
                    self.entity_creation_window = false;
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Etat");
                egui::ComboBox::from_id_salt("Nom de l'état")
                    .selected_text(format!("{:?}", self.selected_state.name()))
                    .show_ui(ui, |ui| {
                        for v in project.get_states(&self.selected_entity.name) {
                            let label = String::from(v.name.as_str());
                            ui.selectable_value(
                                &mut self.selected_state,
                                v,
                                format!("{:?}", label),
                            );
                        }
                    });
                if ui.button("+").clicked() {
                    self.state_creation_window = true;
                }
                if self.create_entity_state(ui, ctx, project, &self.selected_entity.name.clone()) {
                    self.state_creation_window = false;
                }
            });
        });
    }

    fn create_category(&mut self, ui: &mut Ui, ctx: &egui::Context, project: &mut Project) -> bool {
        let current_pos = Pos2::new(100., 100.);
        let mut cancel_window = false;
        egui::Window::new("Création d'une nouvelle catégorie")
            .collapsible(false)
            .current_pos(current_pos)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0., 0.))
            .open(&mut self.category_creation_window)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Nom de la classe d'entité");
                    ui.text_edit_singleline(&mut self.new_category);
                });

                ui.horizontal(|ui| {
                    if ui.button("Sauvegarder").clicked() {
                        project
                            .categories
                            .borrow_mut()
                            .push(EntityCategory::new(&self.new_category));
                        cancel_window = true;
                    }
                    if ui.button("Annuler").clicked() {
                        cancel_window = true;
                    }
                });
            });
        cancel_window
    }

    fn create_entity(&mut self, ui: &mut Ui, ctx: &egui::Context, project: &mut Project) -> bool {
        let current_pos = Pos2::new(100., 100.);
        let mut cancel_window = false;
        egui::Window::new("Création d'une nouvelle entité")
            .collapsible(false)
            .current_pos(current_pos)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0., 0.))
            .open(&mut self.entity_creation_window)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Nom de l'entité");
                    ui.text_edit_singleline(&mut self.name);
                });

                ui.horizontal(|ui| {
                    if ui.button("Sauvegarder").clicked() {
                        project.add_entity(&self.selected_category, &self.name);
                        cancel_window = true;
                    }
                    if ui.button("Annuler").clicked() {
                        cancel_window = true;
                    }
                });
            });
        cancel_window
    }

    fn create_entity_state(
        &mut self,
        ui: &mut Ui,
        ctx: &egui::Context,
        project: &mut Project,
        entity_name: &String,
    ) -> bool {
        let current_pos = Pos2::new(100., 100.);
        let mut cancel_window = false;
        egui::Window::new("Création d'un nouvel état")
            .collapsible(false)
            .current_pos(current_pos)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0., 0.))
            .open(&mut self.state_creation_window)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Nom de l'état");
                    ui.text_edit_singleline(&mut self.new_state);
                });

                ui.horizontal(|ui| {
                    if ui.button("Sauvegarder").clicked() {
                        project.add_entity_state(entity_name, &self.new_state);
                        cancel_window = true;
                    }
                    if ui.button("Annuler").clicked() {
                        cancel_window = true;
                    }
                });
            });
        cancel_window
    }
}
