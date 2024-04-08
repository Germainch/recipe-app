mod app_state;
mod domain;
mod handlers;

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

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port: u16 = 3001;
    let db_connection_str = "postgres://fl0user:FxUDTnLt4EC3@ep-gentle-snowflake-a287b2br.eu-central-1.aws.neon.fl0.io:5432/recipe-db?sslmode=require";

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // build our application with some routes
    let app = Router::new()
        .route("/ingredients", get(handlers::read_ingredient))
        .route("/ingredients", post(handlers::create_ingredient))
        .route("/users", get(handlers::read_user))
        .route("/users", post(handlers::create_user))
        .with_state(pool);

    // run it with hyper
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
