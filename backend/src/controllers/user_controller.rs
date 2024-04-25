use crate::models::user::User;
use axum::{extract, http, Json};
use sqlx::{Error, PgPool};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ptr::hash;
use crate::models::recipe::Recipe;
use crate::models::session::SessionID;

pub async fn read_user(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<i64>,
) -> Result<(http::StatusCode, axum::Json<String>), http::StatusCode> {
    let result = sqlx::query!("SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await;
    match result {
        Ok(user) => Ok((http::StatusCode::OK, axum::Json(user.username))),
        Err(_) => Err(http::StatusCode::NOT_FOUND),
    }
}

pub async fn create_user(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<User>,
) -> Result<http::StatusCode, http::StatusCode> {
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

pub async fn read_saved_recipes(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(session_id): extract::Path<SessionID>,
) -> Result<(http::StatusCode, Json<Vec<Recipe>>), http::StatusCode>{
    let user_id;
    let result = sqlx::query!(
        "SELECT user_id FROM sessions WHERE session_id = $1",
        session_id.session_id
    )
        .fetch_one(&pool)
        .await;
    match result{
        Ok(r) => user_id = r.user_id,
        Err(_) => return Err(http::StatusCode::NOT_FOUND)
    };

    let recipes = sqlx::query_as!(
        Recipe,
        "SELECT * FROM recipes WHERE id IN (SELECT recipe_id FROM saved_recipes WHERE user_id = $1)",
        user_id
    )
        .fetch_all(&pool)
        .await;

    match recipes{
        Ok(r) => Ok((http::StatusCode::OK, Json(r))),
        Err(_) => Err(http::StatusCode::NOT_FOUND)
    }
}

pub async fn update_saved_recipes(
    extract::State(pool): extract::State<PgPool>,
    extract::Path((session_id,recipe_id)): extract::Path<(i64,i64)>,
) -> Result<http::StatusCode,http::StatusCode>{
    let user_id;
    let result = sqlx::query!(
        "SELECT user_id FROM sessions WHERE session_id = $1",
        session_id
    )
        .fetch_one(&pool)
        .await;
    match result{
        Ok(r) => user_id = r.user_id,
        Err(_) => return Err(http::StatusCode::NOT_FOUND)
    };

    let rows_affected = sqlx::query!(
        "INSERT INTO saved_recipes (user_id, recipe_id) VALUES ($1, $2)",
        user_id,
        recipe_id,
    )
        .execute(&pool)
        .await;

    match rows_affected{
        Ok(_) => Ok(http::StatusCode::CREATED),
        Err(_) => Err(http::StatusCode::BAD_REQUEST)
    }
}
