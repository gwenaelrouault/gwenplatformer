use std::cell::RefCell;

use super::{entity_category::EntityCategory, entity_state::EntityState};
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub struct Entity {
    pub name : String,
    pub category : EntityCategory,
    pub states : RefCell<HashMap<String, EntityState>>
}

impl Entity {
    pub fn new(entity_name : &str, entity_category : &EntityCategory) -> Self {
        Entity {
            name : String::from(entity_name),
            category : entity_category.clone(),
            states : RefCell::new(HashMap::new()),
        }
    }

    pub fn default(entity_category : EntityCategory) -> Self {
        Entity {
            name : "Aucune".to_string(),
            category : entity_category,
            states : RefCell::new(HashMap::new()),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}