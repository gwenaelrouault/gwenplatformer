use image::DynamicImage;


#[derive(PartialEq, Debug, Clone)]
pub struct EntityState {
    pub name : String,
    pub frames : Vec<DynamicImage>
}

impl EntityState {
    pub fn new(state_name : &str) -> Self {
        EntityState {
            name : String::from(state_name),
            frames : Vec::new(),
        }
    }

    pub fn default() -> Self {
        EntityState {
            name : "default".to_string(),
            frames : Vec::new(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}