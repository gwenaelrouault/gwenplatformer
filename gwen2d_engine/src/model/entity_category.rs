
#[derive(PartialEq, Debug, Clone)]
pub struct EntityCategory {
    pub name : String,
}

impl EntityCategory {
    pub fn new(category_name : &String) -> Self {
        EntityCategory {
            name : category_name.clone(),
        }
    }

    pub fn default() -> Self {
        EntityCategory {
            name : "Aucune".to_string(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}