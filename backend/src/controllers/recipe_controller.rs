use axum::{extract, http, Json};
use axum::response::Response;
use sqlx::{Error, PgPool};
use sqlx::encode::IsNull::No;
use tracing_subscriber::fmt::format;
use crate::controllers::user_controller::read_user;
use crate::models::ingredient::Ingredient;
use crate::models::recipe::Recipe;
use crate::models::user::User;
pub async fn read_recipes(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(recipe_name): extract::Path<String>,
)
    -> Result<(http::StatusCode, Json<Vec<Recipe>>), http::StatusCode>
{
    let pattern = format!("%{}%", recipe_name);

    let result = sqlx::query_as!(
        Recipe,
        "SELECT * FROM recipes WHERE name LIKE ($1)",
        pattern,
    )
        .fetch_all(&pool)
        .await;
    match result {
        Ok(recipes) => Ok((http::StatusCode::OK, Json(recipes))),
        Err(e) => {
            println!("Error: {}", e);
            Err(http::StatusCode::NOT_FOUND)
        },
    }
}


pub async fn create_recipe(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<String>,
    extract::State(session_id): extract::State<i64>,
) -> Result<http::StatusCode, http::StatusCode> {

    let recipe = serde_json::from_str::<Recipe>(&payload).unwrap();
    let userid: i64;
    let user = sqlx::query!(
        "SELECT user_id FROM sessions WHERE session_id = $1",
        session_id
    )
        .fetch_optional(&pool)
        .await;

    match user {
        Ok(user) => {
            match user {
                None => return Err(http::StatusCode::UNAUTHORIZED),
                Some(u) => userid = u.user_id
            }
        },
        Err(_) => {
            return Err(http::StatusCode::UNAUTHORIZED);
    }};

    let rows_affected = sqlx::query!(
        "INSERT INTO recipes (name, steps, created_by) VALUES ($1, $2, $3) RETURNING id",
        recipe.name(),
        recipe.steps(),
        userid
    )
        .fetch_one(&pool)
        .await;

    match rows_affected {
        Ok(_) => Ok(http::StatusCode::CREATED),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn delete_recipe(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<i64>,
) -> http::StatusCode {
    let result = sqlx::query!(
        "DELETE FROM recipes WHERE id = $1",
        id
    )
        .execute(&pool)
        .await;
    match result {
        Ok(_) => http::StatusCode::NO_CONTENT,
        Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR
    }
}