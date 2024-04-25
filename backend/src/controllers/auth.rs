use axum::extract::{Path, State};
use axum::{debug_handler, http, Json};
use axum::http::{HeaderMap, HeaderValue};
use sqlx::PgPool;
use crate::models::user::User;
use crate::utils::hash;
use crate::utils::hash::hash_password;
use crate::middlewares::session::*;
use crate::models::ingredient::Ingredient;
use crate::models::session::SessionID;

#[debug_handler]
pub async fn sign_in(
    State(pool) : State<PgPool>,
    axum::Json(payload): axum::Json<User>
) -> Result<(http::StatusCode, axum::Json<i64>), http::StatusCode>{

    let hash = hash_password(&payload.password_hash);


    let is_user = sqlx::query!(
        "SELECT * FROM users WHERE username = $1 AND password = $2",
        payload.name,
        hash,
    )
        .fetch_one(&pool)
        .await;

    match is_user {
        Ok(u) => create_session(State(pool), u.id).await,
        Err(_) => Err(http::StatusCode::UNAUTHORIZED)
    }
}
#[debug_handler]
pub async fn sign_up(
    State(pool) : State<PgPool>,
    axum::Json(payload): axum::Json<User>
) -> Result<(http::StatusCode, Json<i64>), http::StatusCode>{

    let hash = hash_password(&payload.password_hash);

    /// Check if the user already exists
    let is_user = sqlx::query!(
        "SELECT * FROM users WHERE username = $1",
        payload.name,
    )
        .fetch_optional(&pool)
        .await;

    /// if user exists, return a BAD_REQUEST status code
    /// if user is successfully inserted, return a CREATED status code with the session id
    match is_user {
        Ok(u) => match u {
            Some(_) => Err(http::StatusCode::BAD_REQUEST),
            None => {
                let user = sqlx::query!(
                    "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id",
                    payload.name,
                    hash,
                )
                    .fetch_one(&pool)
                    .await;
                match user {
                    Ok(u) => create_session(State(pool), u.id).await,
                    Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        },
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[debug_handler]
pub async fn sign_out(
    State(pool) : State<PgPool>,
    Path(session_id) : Path<SessionID>
) -> http::StatusCode {
    let sid: i64 = session_id.session_id;
    let result = invalidate_session(State(pool), sid).await;
    result
}
