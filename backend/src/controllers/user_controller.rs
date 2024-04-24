
use axum::{extract, http};
use sqlx::PgPool;



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

pub async fn create_user() -> http::StatusCode {
    todo!("Implement create_user");

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