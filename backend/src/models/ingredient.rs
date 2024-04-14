use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    id: i32,
    name: String,
}

impl Ingredient {
    pub fn new(id: i32, name: &str) -> Self {
        Ingredient { id, name : name.to_string() }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
