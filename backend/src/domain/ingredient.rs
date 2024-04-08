use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
}

impl Ingredient {
    pub fn new(name: String) -> Self {
        Ingredient { id: 1, name }
    }
}
