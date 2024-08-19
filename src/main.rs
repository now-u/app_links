pub mod dao;
mod link_router;
mod management;

use link_router::create_link_router_service;
use management::create_management_service;
use poem::{listener::TcpListener, EndpointExt, Result, Route};

use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let pool =
        sqlx::postgres::PgPool::connect("postgresql://postgres:postgres@127.0.0.1:9091/polylink")
            .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Route::new()
        .nest("/api", create_management_service())
        .nest("/", create_link_router_service())
        .data(pool);

    poem::Server::new(TcpListener::bind("0.0.0.0:9090"))
        .run(app)
        .await?;

    Ok(())
}
