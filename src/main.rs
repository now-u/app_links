mod crawler;
pub mod dao;
mod link_router;
mod management;

use std::env;

use link_router::create_link_router_service;
use management::create_management_service;
use poem::{listener::TcpListener, EndpointExt, Result, Route};
use url::Url;

use sqlx::Postgres;
use tracing_subscriber;

#[derive(Clone)]
struct AppContext {
    pool: sqlx::Pool<Postgres>,
    base_url: Url,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let postgres_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPool::connect(&postgres_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let base_url = env::var("BASE_URL").expect("BASE_URL must be defined");
    let base_url = url::Url::parse(&base_url).expect("BASE_URL must be a valid url");

    let app = Route::new()
        .nest("/api", create_management_service(&base_url))
        .nest("/", create_link_router_service())
        .data(AppContext { pool, base_url });

    poem::Server::new(TcpListener::bind("0.0.0.0:9090"))
        .run(app)
        .await?;

    Ok(())
}
