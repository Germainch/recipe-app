mod controllers;
mod middlewares;
mod models;
mod utils;

use crate::controllers::{auth, ingredient_controller, recipe_controller, user_controller};
use axum::http::{HeaderValue, Method};
use axum::routing::post;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::get,
    Router,
};
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env::current_dir;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    println!(
        "{}",
        std::env::var("DATABASE_URL").unwrap_or("ALOOO".parse().unwrap())
    );
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
        .route(
            "/ingredients/:id",
            get(ingredient_controller::read_ingredient),
        )
        .route(
            "/ingredients/containsStr/:str",
            get(ingredient_controller::read_ingredient_contains),
        )
        .route("/users/:id", get(user_controller::read_user))
        .route("/users", post(user_controller::create_user))
        .route(
            "/recipes/search-by-ingredients",
            post(recipe_controller::read_recipes_by_ingredients),
        )
        .route(
            "/recipes/search-by-name/:recipe_name",
            get(recipe_controller::read_recipes),
        )

        .route("/recipes/saved/session/:session_id/recipe/:recipe_id", post(user_controller::update_saved_recipes))
        .route("/recipes/saved/:session_id", get(user_controller::read_saved_recipes))
        .route("/recipes/create/:session_id", post(recipe_controller::create_recipe))

        .route("/login", post(auth::sign_in))
        .route("/signup", post(auth::sign_up))
        .route("/logout/:session_id", post(auth::sign_out))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
                .allow_headers(Any),
        )
        .with_state(pool);

    println!("Running on address {}:{}", Ipv4Addr::LOCALHOST, port);
    // run it
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
