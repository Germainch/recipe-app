use axum::extract::{Path, State};
use axum::http;
use sqlx::PgPool;
use crate::models::ingredient::Ingredient;

pub async fn create_recipe_ingredients(
    pool : PgPool,
    ingredients : Vec<Ingredient>,
    recipe_id: i64,
) -> Result<http::StatusCode, http::StatusCode>{
    for ingredient in ingredients {
        let rows_affected = sqlx::query!(
            "INSERT INTO recipe_ingredients (recipe_id, ingredient_id) VALUES ($1, $2)",
            recipe_id,
            ingredient.id,
        )
        .execute(&pool)
        .await;
        match rows_affected {
            Ok(_) => continue,
            Err(_) => return Err(http::StatusCode::BAD_REQUEST),
        }
    }
    Ok(http::StatusCode::CREATED)
}

