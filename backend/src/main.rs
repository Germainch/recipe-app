
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
use crate::controllers::user_controller;

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

    let port: u16 = 3001;

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("can't connect to database");

    // routes
    let app = Router::new()
        .route("/ingredients:id", get(user_controller::read_ingredient))
        .route("/ingredients:containsSTR", get(user_controller::read_ingredient))
        .route("/ingredients", get(user_controller::read_ingredient))
        .route("/ingredients", post(user_controller::create_ingredient))
        .route("/users", get(user_controller::read_user))
        .route("/users", post(user_controller::create_user))
        .with_state(pool);

    // run it
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
