use std::iter;

use rand::Rng;
use sqlx::prelude::FromRow;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error with configuration: {0}")]
    UnknownError(#[source] Box<dyn std::error::Error + 'static + Send + Sync>),
}

#[derive(Debug, Clone, Eq, PartialEq, Default, FromRow)]
pub struct Link {
    pub id: uuid::Uuid,
    pub url_path: String,

    pub title: String,
}

fn generate_url_path() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(24).collect()
}

pub async fn create_link(pool: &sqlx::Pool<sqlx::Postgres>, title: &str) -> Result<Link, Error> {
    let url_path = generate_url_path();

    tracing::info!("Creating new link url_path={url_path}");

    let result: Result<Link, _> = sqlx::query_as(
        "INSERT INTO links (url_path, title) VALUES ($1, $2) RETURNING id, url_path, title",
    )
    .bind(url_path)
    .bind(title)
    .fetch_one(pool)
    .await;

    result.map_err(|err| {
        tracing::error!("Error whilst creating link err={err}");
        Error::UnknownError(Box::new(err))
    })
}

pub async fn list_links(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<Link>, Error> {
    tracing::info!("Fetching all links");

    let result = sqlx::query_as::<_, Link>("SELECT id, url_path, title FROM links")
        .fetch_all(pool)
        .await;

    result.map_err(|err| {
        tracing::error!("Error whilst listing link err={err}");
        Error::UnknownError(Box::new(err))
    })
}

pub async fn get_link(
    pool: &sqlx::Pool<sqlx::Postgres>,
    link_id: &uuid::Uuid,
) -> Result<Option<Link>, Error> {
    tracing::info!("Getting link by id link_id={link_id}");

    let result = sqlx::query_as::<_, Link>("SELECT id, url_path, title FROM links WHERE id = $1")
        .bind(link_id)
        .fetch_optional(pool)
        .await;

    result.map_err(|err| {
        tracing::error!("Error whilst creating link err={err}");
        Error::UnknownError(Box::new(err))
    })
}

pub async fn get_link_by_url_path(
    pool: &sqlx::Pool<sqlx::Postgres>,
    url_path: &str,
) -> Result<Option<Link>, Error> {
    tracing::info!("Getting link by url path url_path={url_path}");

    tracing::info!("Running query");

    let result = sqlx::query_as::<_, Link>("SELECT id, url_path, title FROM links WHERE url_path = $1")
        .bind(url_path)
        .fetch_optional(pool)
        .await;

    result.map_err(|err| {
        tracing::error!("Error whilst creating link err={err}");
        Error::UnknownError(Box::new(err))
    })
}
