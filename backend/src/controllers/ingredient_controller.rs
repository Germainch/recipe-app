use axum::{extract, http};
use axum::http::Request;
use sqlx::PgPool;
use rand::random;
use crate::models::ingredient::{Ingredient};

pub async fn read_ingredient(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<i32>,
) -> Result<(http::StatusCode, axum::Json<Vec<Ingredient>>), http::StatusCode> {

    let result = sqlx::query_as!(
        Ingredient,
        "SELECT * FROM ingredients",
    )
        .fetch_all(&pool)
        .await;

    match result {
        Ok(ingredients) => Ok((http::StatusCode::OK, axum::Json(ingredients))),
        Err(_) => Err(http::StatusCode::NOT_FOUND)
    }

}

pub async fn create_ingredient(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<Ingredient>,
) -> Result<(http::StatusCode, axum::Json<Ingredient>), http::StatusCode> {

    let ingredient = Ingredient::new( 1 , payload.name() );
    let result = sqlx::query!(
        "INSERT INTO ingredients (name) VALUES ($1)",
        ingredient.name()
    )
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok((http::StatusCode::OK, axum::Json(ingredient))),
        Err(e) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }

}

pub async fn update_ingredient() -> http::StatusCode {
    http::StatusCode::OK
}

pub async fn delete_ingredient() -> http::StatusCode {
    http::StatusCode::OK
}