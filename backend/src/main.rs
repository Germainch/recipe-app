mod handlers;
mod domain;
mod app_state;

use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::get,
    Router,
};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::time::Duration;


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    let port : u16 = std::env::var("PORT").unwrap().parse::<u16>().unwrap();
    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // build our application with some routes
    let app = Router::new()
        .with_state(pool);

    // run it with hyper
    let listener = TcpListener::bind(SocketAddrV4::new( Ipv4Addr::LOCALHOST, port )).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}