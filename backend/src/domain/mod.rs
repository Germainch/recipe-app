pub(crate) mod ingredient;
mod user;

// Path: backend/src/handlers/mod.rs

pub struct Ingredient {
    pub id: u32,
    pub name: String,
}

pub async fn read_ingredient(id: u32) -> Ingredient {
    Ingredient {
        id: 1,
        name: "Flour".to_string(),
    }
}

pub async fn create_ingredient(ingredients: Vec<Ingredient>) -> Vec<Ingredient> {
    ingredients
}
