
mod models;
mod controllers;
mod middlewares;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::get,
    Router,
};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::time::Duration;
use axum::routing::post;
use dotenv::dotenv;
use std::env::current_dir;
use axum::http::{HeaderValue, Method};
use crate::controllers::{ingredient_controller, user_controller};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // imports .env file into process environment
    dotenv().ok();
    // sets up tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("{}",  std::env::var("DATABASE_URL").unwrap_or("ALOOO".parse().unwrap()) );
    // set up port
    let port: u16 = 3001;

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("can't connect to database");

    // routes
    let app = Router::new()
        .route("/ingredients/:id", get(ingredient_controller::read_ingredient))
        .route("/ingredients/containsStr/:str", get(ingredient_controller::read_ingredient_contains))
        .route("/ingredients", get(ingredient_controller::read_all))
        .route("/users", get(user_controller::read_user))
        .route("/users", post(user_controller::create_user))
        .route("/recipes/search-by-ingredients", get(recipe_controller::search_by_ingredients))
        .route("/recipes/search-by-name?ingredients", get(recipe_controller::search_by_name))
        .route("/recipes/saved/?sessionID=", get(user_controller::update_saved_recipes))
        .route("/recipes/saved/?sessionID=", post(recipe_controller::create_recipe))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        )
        .with_state(pool);

    println!("Running on address {}:{}",Ipv4Addr::LOCALHOST, port);
    // run it
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
