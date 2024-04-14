

pub async fn validate_session() -> Result<axum::Json<&'static str>, axum::http::StatusCode> {
    Ok(axum::Json(&"Session is valid"))
}