
use axum::{extract, http};
use sqlx::PgPool;
use crate::models::ingredient::{CreateIngredient, Ingredient};


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

pub async fn update_user() -> http::StatusCode {
    http::StatusCode::OK
}

pub async fn delete_user() -> http::StatusCode {
    http::StatusCode::OK
}