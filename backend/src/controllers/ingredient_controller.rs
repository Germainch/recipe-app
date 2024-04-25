use crate::models::ingredient::Ingredient;
use axum::http::Request;
use axum::{extract, http};
use rand::random;
use sqlx::PgPool;

pub async fn read_ingredient(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<i64>,
) -> Result<(http::StatusCode, axum::Json<Ingredient>), http::StatusCode> {
    let result = sqlx::query_as!(Ingredient, "SELECT * FROM ingredients WHERE id = $1", id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(ingredients) => Ok((http::StatusCode::OK, axum::Json(ingredients))),
        Err(_) => Err(http::StatusCode::NOT_FOUND),
    }
}

pub async fn read_ingredient_contains(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(str): extract::Path<String>,
) -> Result<(http::StatusCode, axum::Json<Vec<Ingredient>>), http::StatusCode> {
    let pattern = format!("%{}%", str);
    let result = sqlx::query_as!(
        Ingredient,
        "SELECT * FROM ingredients WHERE name LIKE $1",
        pattern
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(ingredients) => Ok((http::StatusCode::OK, axum::Json(ingredients))),
        Err(_) => Err(http::StatusCode::NOT_FOUND),
    }
}
