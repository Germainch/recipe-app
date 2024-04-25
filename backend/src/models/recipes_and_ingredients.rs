use serde::{Deserialize, Serialize};
use crate::models::ingredient::Ingredient;
use crate::models::recipe::Recipe;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeAndIngredients {
    pub recipe: Recipe,
    pub ingredients: Vec<Ingredient>,
}