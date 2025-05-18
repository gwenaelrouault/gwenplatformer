use std::{cell::RefCell, collections::HashMap};
use log::{debug, error, log_enabled, info, Level};
use super::{entity::Entity, entity_category::EntityCategory, entity_state::EntityState};

pub struct Project {
    pub name : String,
    pub categories : RefCell<Vec<EntityCategory>>,
    pub entities : RefCell<HashMap<String, Entity>>,
}

impl Project {
    pub fn new(project_name : String) -> Self {
        Project {
            name : project_name,
            categories : RefCell::new(vec![EntityCategory::default()]),
            entities : RefCell::new(HashMap::new()),
        }
    }

    pub fn add_entity(&mut self, category : &EntityCategory, name : &str) {
        self.entities.borrow_mut().insert(String::from(name), Entity::new(name, category));
    }

    pub fn add_entity_state(&mut self, entity_name : &str, state_name : &str) {
        match (self.entities.borrow_mut().get_mut(entity_name)) {
            Some(entity) => { entity.states.borrow_mut().insert(String::from(state_name), EntityState::new(state_name)); },
            None =>  { info!("Error while creating state {}", state_name); }
        }
    }

    pub fn get_states(&mut self, entity_name : &str) -> Vec<EntityState> {
        match self.entities.borrow_mut().get_mut(entity_name) {
            Some(entity) => entity.states.borrow().values().cloned().collect(),
            None =>  Vec::new()
        }
    }
}