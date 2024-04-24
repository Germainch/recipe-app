use std::hash::{DefaultHasher, Hash, Hasher};
use std::ptr::hash;
use axum::{extract, http};
use sqlx::PgPool;
use crate::models::user::User;


pub async fn read_user(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<i64>,
) -> Result<(http::StatusCode, axum::Json<String>), http::StatusCode> {

    let result = sqlx::query!("SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await;
    match result {
        Ok(user) => Ok( ( http::StatusCode::OK, axum::Json(user.username) ) ),
        Err(_) => Err( http::StatusCode::NOT_FOUND ),
    }
}

pub async fn create_user(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<User>,
) -> Result<http::StatusCode, http::StatusCode>{

    /// Extract the user from the payload
    let mut user = payload;



    /// Hash the password
    let mut hasher = DefaultHasher::new();
    user.password_hash.hash(&mut hasher);
    user.set_password_hash(hasher.finish().to_string());

    println!("Created {:?}", user);
    /// Insert the user into the database
    /// If the user already exists, return a BAD_REQUEST status code
    /// If the user is successfully inserted, return a CREATED status code
    let rows_affected = sqlx::query!(
        "INSERT INTO users (username, password) VALUES ($1, $2)",
        user.name,
        user.password_hash
    )
        .execute(&pool)
        .await;
    match rows_affected {
        Ok(_) => Ok(http::StatusCode::CREATED),
        Err(_) => Err(http::StatusCode::BAD_REQUEST),
    }
}

pub async fn update_user() -> http::StatusCode {
    todo!("Implement update_user");
}

pub async fn delete_user() -> http::StatusCode {
    todo!("Implement delete_user");
}

pub async fn update_saved_recipes(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<String>,
    extract::Path(session_id) : extract::Path<String>,
) -> http::StatusCode {
    println!("session_id: {}", session_id);
    http::StatusCode::OK
}