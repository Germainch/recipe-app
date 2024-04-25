use axum::extract::State;
use axum::{extract, http, Json};
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, PgPool, Pool};

/// `validate_session` is an asynchronous function that checks the validity of a user's session.
///
/// # Returns
/// This function returns a `Result` type. On successful validation, it returns a JSON response with the message "Session is valid".
/// If the validation fails, it returns an HTTP status code.
///
/// # Example
/// ```rust
/// let result = validate_session().await;
/// match result {
///     Ok(response) => println!("{}", response.0),  // Prints "Session is valid"
///     Err(status_code) => println!("{}", status_code),  // Prints the HTTP status code
/// }
/// ```
pub async fn validate_session(
    State(pool): State<PgPool>,
    session_id: i64,
) -> Result<http::StatusCode, axum::http::StatusCode> {
    let response = sqlx::query!("SELECT * FROM sessions WHERE session_id = $1", session_id)
        .fetch_optional(&pool)
        .await;
    match response {
        Ok(session) => match session {
            Some(_) => Ok(http::StatusCode::OK),
            None => Err(http::StatusCode::UNAUTHORIZED),
        },
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// `create_session` is an asynchronous function that creates a new session for a user.
///
/// # Returns
/// This function returns an HTTP status code. On successful creation, it returns a status code of `CREATED`.
///
/// # Example
/// ```rust
/// let status = create_session().await;
/// println!("{}", status);  // Prints "201 Created"
/// ```
pub async fn create_session(
    extract::State(pool): extract::State<PgPool>,
    user_id: i64,
) -> Result<(http::StatusCode, Json<i64>), http::StatusCode> {

    let session_id = 0i64;
    let response = sqlx::query!(
        "INSERT INTO sessions (user_id) VALUES ($1) RETURNING session_id",
        user_id,
    )
    .fetch_one(&pool)
    .await;
    match response {
        Ok(r) => Ok((http::StatusCode::CREATED, Json::from(r.session_id))),
        Err(e) => {
            println!("Error: {}", e);
            Err(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// `invalidate_session` is an asynchronous function that invalidates a user's session.
///
/// # Returns
/// This function returns an HTTP status code. On successful invalidation, it returns a status code of `OK`.
///
/// # Example
/// ```rust
/// let status = invalidate_session().await;
/// println!("{}", status);  // Prints "200 OK"
/// ```
pub async fn invalidate_session(
    State(pool): State<PgPool>,
    session_id: i64,
) -> axum::http::StatusCode {
    let response = sqlx::query!("DELETE FROM sessions WHERE session_id = $1", session_id)
        .execute(&pool)
        .await;
    match response {
        Ok(_) => http::StatusCode::OK,
        Err(e) => {
            println!("Error: {}", e);
            http::StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
