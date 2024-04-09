use crate::domain::ingredient::{CreateIngredient, Ingredient};
use axum::{extract, http};
use sqlx::PgPool;

/****************************************************
 ****************** INGREDIENTS *********************
 ****************************************************/
pub async fn read_ingredient(
    extract::State(pool): extract::State<PgPool>,
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
    axum::Json(payload): axum::Json<CreateIngredient>,
) -> Result<(http::StatusCode, axum::Json<CreateIngredient>), http::StatusCode> {

    let ingredient = CreateIngredient::new(payload.name);
    let result = sqlx::query!(
        "INSERT INTO ingredients (name) VALUES ($1)",
        &ingredient.name
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok((http::StatusCode::OK, axum::Json(ingredient))),
        Err(e) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/****************************************************
 ****************** USERS ***************************
 ****************************************************/

pub async fn read_user(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<String>,
) -> Result<(http::StatusCode, axum::Json<String>), http::StatusCode> {
    let user_name = payload;
    let result = sqlx::query!("SELECT * FROM users WHERE username = $1", user_name)
        .fetch_one(&pool)
        .await;
    match result {
        Ok(user) => Ok((http::StatusCode::OK, axum::Json(user.username))),
        Err(_) => Ok((
            http::StatusCode::NOT_FOUND,
            axum::Json(user_name.to_string()),
        )),
    }
}

pub async fn create_user() -> http::StatusCode {
    http::StatusCode::CREATED
}