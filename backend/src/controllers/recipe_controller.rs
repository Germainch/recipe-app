use std::io::ErrorKind;
use crate::controllers::user_controller::read_user;
use crate::models::ingredient::Ingredient;
use crate::models::recipe::Recipe;
use crate::models::user::User;
use axum::response::Response;
use axum::{debug_handler, extract, http, Json};
use axum::extract::Path;
use serde::de::value::StringDeserializer;
use serde_json::{json, Value};
use sqlx::encode::IsNull::No;
use sqlx::{Error, PgPool};
use tracing_subscriber::fmt::format;
use crate::controllers::recipe_ingredients::create_recipe_ingredients;
use crate::models::recipes_and_ingredients::RecipeAndIngredients;

use crate::models::session::SessionID;

pub async fn read_recipes(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(recipe_name): extract::Path<String>,
) -> Result<(http::StatusCode, Json<Vec<Recipe>>), http::StatusCode> {

    let mut fetched_recipes: Vec<Recipe>;
    let pattern = format!("%{}%", recipe_name);

    let result = sqlx::query_as!(
        Recipe,
        "SELECT * FROM recipes WHERE name LIKE ($1)",
        pattern,
    )
    .fetch_all(&pool)
    .await;
    match result {
        Ok(recipes) => fetched_recipes = recipes,
        Err(e) => {
            println!("Error: {}", e);
            return Err(http::StatusCode::NOT_FOUND);
        }
    };
    let extension = fetch_recipes_by_name(&recipe_name).await;
    match extension{
        Ok(mut recipes) => {
            fetched_recipes.append(&mut recipes);
        },
        Err(e) => {println!("Error: {}", e);}
    };

    match fetched_recipes.len() {
        0 => Err(http::StatusCode::NOT_FOUND),
        _ => Ok((http::StatusCode::OK, Json(fetched_recipes)))
    }
}

pub async fn fetch_recipes_by_name(name: &str) -> Result<Vec<Recipe>, Error >{
    let api_key = std::env::var("API_KEY").unwrap();
    let url = format!("https://api.spoonacular.com/recipes/complexSearch?apiKey={}&query={}", api_key, name);
    let res = reqwest::get(url).await;
    match res {
        Ok(r) => {
            let body = r.json::<Value>().await.unwrap_or_default();

            // can be None if no results are found
            let mut results = body.get("results");
            let arr;
            match results{
                Some(a) => arr = a.as_array().unwrap(),
                None => return Ok(vec![])
            }


            let mut recipes: Vec<Recipe> = Vec::new();
            for recipe in arr {
                let id = recipe.get("id").unwrap();
                let summary = fetch_recipe_summary(id.as_i64().unwrap()).await.unwrap();
                recipes.push(summary);
            }
            Ok(recipes)
        },
        Err(e) => {Err(Error::from(std::io::Error::new(ErrorKind::AlreadyExists, "cannot fetch recipes by name")))}
    }
}

pub async fn read_recipes_by_ingredients(
    extract::State(pool): extract::State<PgPool>,
    extract::Json(ingredients): extract::Json<Vec<Ingredient>>,
) -> Result<(http::StatusCode, Json<Vec<Recipe>>), http::StatusCode> {

    // filter ingredients to get only the ids
    let ingredients_ids: Vec<i64> = ingredients.iter().map(|i| i.id()).collect();

    let mut recipes_fetched: Vec<Recipe> = vec![];

    let result = sqlx::query_as!(
        Recipe,
        "SELECT * FROM recipes WHERE id IN \
            (SELECT recipe_id FROM recipe_ingredients WHERE ingredient_id = ANY($1) \
            GROUP BY recipe_id
            HAVING COUNT(DISTINCT ingredient_id) = $2)\
        ",
        &ingredients_ids,
        ingredients_ids.len() as i64,
    )
        .fetch_all(&pool)
        .await;

    match result {
        Ok(recipes) => recipes_fetched = recipes,
        Err(e) => {println!("Error: {}", e);}
    };

    // fetch recipes from the Spoonacular API
    let ingredients_str = ingredients
        .iter()
        .map(|i| i.name())
        .collect::<Vec<&str>>()
        .join(",");

    println!("Ingredients: {}", ingredients_str);
    let api_key = std::env::var("API_KEY").unwrap();
    let url = format!("https://api.spoonacular.com/recipes/findByIngredients?apiKey={}&ingredients={}", api_key, ingredients_str);
    let api_response = reqwest::get(url)
        .await;

    match api_response {
        Ok(response) => {
            let empty = Vec::new();
            let body = response.json::<Value>().await.unwrap_or_default();

            let arr = body.as_array().unwrap_or(&empty);


            let mut recipe_ids: Vec<i64> = Vec::new();
            println!("{}", body);
            for r in arr.iter() {
                let id = r.get("id").unwrap();
                println!("{}", id);
                recipe_ids.push(id.as_i64().unwrap());
            }
            for id in recipe_ids {
                let summary = fetch_recipe_summary(id).await.unwrap();
                recipes_fetched.push(summary);
            }
        }
        Err(e) => { println!("Error: {}", e); }
    }

    match recipes_fetched.len() {
        0 => Err(http::StatusCode::NOT_FOUND),
        _ => Ok((http::StatusCode::OK, Json(recipes_fetched)))
    }
}

pub async fn fetch_recipe_summary(id: i64) -> Result<Recipe, Error>{

    let api_key = std::env::var("API_KEY").unwrap();
    let url = format!("https://api.spoonacular.com/recipes/{}/summary/?apiKey={}", id, api_key);
    let res = reqwest::get(url).await;
    match res{
        Ok(response) => {
            let body = response.json::<Value>().await.unwrap_or_default();
            let steps = body.get("summary").unwrap();
            let name = body.get("title").unwrap();
            let id = body.get("id").unwrap();
            let recipe = Recipe::new(id.as_i64().unwrap(), name.as_str().unwrap(), steps.as_str().unwrap(), 0);
            Ok(recipe)
        },
        Err(e) => { println!("Error: {}", e); Err(Error::from(std::io::Error::new(ErrorKind::AlreadyExists, "aaa"))) }
    }
}

#[debug_handler]
pub async fn create_recipe(
    extract::State(pool): extract::State<PgPool>,
    Path(session_id): Path<SessionID>,
    Json(payload): Json<RecipeAndIngredients>,
) -> Result<http::StatusCode, http::StatusCode> {

    let recipe = payload.recipe;
    let ingredients = payload.ingredients;

    let userid: i64;
    let user = sqlx::query!(
        "SELECT user_id FROM sessions WHERE session_id = $1",
        session_id.session_id
    )
    .fetch_optional(&pool)
    .await;

    match user {
        Ok(user) => match user {
            None => return Err(http::StatusCode::UNAUTHORIZED),
            Some(u) => userid = u.user_id,
        },
        Err(_) => {
            return Err(http::StatusCode::UNAUTHORIZED);
        }
    };

    let rows_affected = sqlx::query!(
        "INSERT INTO recipes (name, steps, created_by) VALUES ($1, $2, $3) RETURNING id",
        recipe.name(),
        recipe.steps(),
        userid
    )
    .fetch_one(&pool)
    .await;

    match rows_affected {
        Ok(i) => {
            create_recipe_ingredients(pool, ingredients, i.id).await.unwrap();
            Ok(http::StatusCode::CREATED)
        },
        Err(_) => Err(http::StatusCode::IM_USED),
    }
}

pub async fn delete_recipe(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<i64>,
) -> http::StatusCode {
    let result = sqlx::query!("DELETE FROM recipes WHERE id = $1", id)
        .execute(&pool)
        .await;
    match result {
        Ok(_) => http::StatusCode::NO_CONTENT,
        Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}