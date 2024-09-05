mod crawler;
pub mod dao;
mod link_router;
mod management;

use std::env;

use link_router::create_link_router_service;
use management::create_management_service;
use poem::{endpoint::make_sync, get, listener::TcpListener, EndpointExt, Response, Result, Route};
use tracing::info;
use url::Url;

use sqlx::Postgres;

#[derive(Clone)]
struct AppContext {
    pool: sqlx::Pool<Postgres>,
    api_key: String,
    base_url: Url,
}

#[derive(Clone)]
struct FallbackData {
    web_fallback: Url,
    ios_fallback: Url,
    android_fallback: Url,
}

fn get_fallback_from_env(env_name: &str) -> Url {
    let env_value = env::var(env_name).unwrap_or_else(|_| panic!("{env_name} must be defined"));
    Url::parse(&env_value).unwrap_or_else(|_| panic!("{env_name} must be a valid url"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Connecting to postgres");
    let postgres_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPool::connect(&postgres_url).await?;
    info!("Connected to postgres");

    info!("Running migrations");
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Migrations complete");

    let base_url = env::var("BASE_URL").expect("BASE_URL must be defined");
    let base_url = url::Url::parse(&base_url).expect("BASE_URL must be a valid url");

    let api_key = env::var("API_KEY").expect("API_KEY must be defined");

    info!("Creating app");
    let app = Route::new()
        .at(
            "/.well-known/assetlinks.json",
            get(make_sync(move |_| {
                Response::builder()
                    .content_type("application/json")
                    .body(include_str!("assetlinks.json"))
            })),
        )
        .at(
            "/.well-known/apple-app-site-association",
            get(make_sync(move |_| {
                Response::builder()
                    .content_type("application/json")
                    .body(include_str!("apple-app-site-association.json"))
                })),
            )
        .nest("/api", create_management_service(&base_url))
        .nest("/", create_link_router_service())
        .data(AppContext {
            pool,
            base_url,
            api_key,
        })
        .data(FallbackData {
            web_fallback: get_fallback_from_env("WEB_FALLBACK_URL"),
            android_fallback: get_fallback_from_env("ANDROID_FALLBACK_URL"),
            ios_fallback: get_fallback_from_env("IOS_FALLBACK_URL"),
        });

    info!("Starting server");
    poem::Server::new(TcpListener::bind("0.0.0.0:9090"))
        .run(app)
        .await?;

    Ok(())
}
