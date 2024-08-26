use std::iter;

use poem_openapi::Object;
use rand::Rng;
use sqlx::prelude::FromRow;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error with configuration: {0}")]
    UnknownError(#[source] Box<dyn std::error::Error + 'static + Send + Sync>),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::UnknownError(Box::new(value))
    }
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct LinkData {
    pub title: String,
    pub description: String,
    pub image_url: String,

    pub web_destination: String,
    pub ios_destination: String,
    pub android_destination: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, FromRow)]
pub struct Link {
    pub id: uuid::Uuid,
    pub link_path: String,

    pub title: String,
    pub description: String,
    pub image_url: String,

    pub web_destination: String,
    pub ios_destination: String,
    pub android_destination: String,
}

impl Link {
    // TODO Should return URI
    fn get_url(base_url: &str) -> String {
        todo!()
    }
}

fn generate_link_path() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(24).collect()
}

const LINKS_SELECT_FIELDS: &str =
    "id, link_path, title, description, image_url, android_destination, ios_destination, web_destination";

pub async fn create_link(
    pool: &sqlx::Pool<sqlx::Postgres>,
    input: LinkData,
) -> Result<Link, Error> {
    let link_path = generate_link_path();

    tracing::info!("Creating new link link_path={link_path}");

    let result: Result<Link, _> = sqlx::query_as(&format!(
        "INSERT INTO links (link_path, title, description, image_url, android_destination, ios_destination, web_destination) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING {LINKS_SELECT_FIELDS}"
    ))
        .bind(link_path)
        .bind(input.title)
        .bind(input.description)
        .bind(input.image_url)
        .bind(input.web_destination)
        .bind(input.ios_destination)
        .bind(input.android_destination)
        .fetch_one(pool)
        .await;

    result.map_err(|err| {
        tracing::error!("Error whilst creating link err={err}");
        Error::UnknownError(Box::new(err))
    })
}

pub async fn update_link(
    pool: &sqlx::Pool<sqlx::Postgres>,
    link_id: &uuid::Uuid,
    link_data: LinkData,
) -> Result<Option<Link>, Error> {
    tracing::info!("Updating link link_id={link_id}");

    let result: Result<Option<Link>, _> = sqlx::query_as(&format!(
        "UPDATE links SET title = $1, description = $2, image_url = $3, android_destination = $4, ios_destination = $5, web_destination = $6 WHERE id = $7 RETURNING {LINKS_SELECT_FIELDS}"
    ))
        .bind(link_data.title)
        .bind(link_data.description)
        .bind(link_data.image_url)
        .bind(link_data.web_destination)
        .bind(link_data.ios_destination)
        .bind(link_data.android_destination)
        .bind(link_id)
        .fetch_optional(pool)
        .await;

    result.map_err(|err| {
        tracing::error!("Error whilst creating link err={err}");
        Error::UnknownError(Box::new(err))
    })
}

pub async fn list_links(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<Link>, Error> {
    tracing::info!("Fetching all links");

    let result = sqlx::query_as::<_, Link>(&format!("SELECT {LINKS_SELECT_FIELDS} FROM links"))
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

    let result = sqlx::query_as::<_, Link>(&format!("SELECT {LINKS_SELECT_FIELDS} FROM links WHERE id = $1"))
        .bind(link_id)
        .fetch_optional(pool)
        .await;

    result.map_err(|err| {
        tracing::error!("Error whilst fetching link link_id={link_id} err={err}");
        Error::UnknownError(Box::new(err))
    })
}

pub async fn get_link_by_link_path(
    pool: &sqlx::Pool<sqlx::Postgres>,
    link_path: &str,
) -> Result<Option<Link>, Error> {
    tracing::info!("Getting link by url path link_path={link_path}");

    let result = sqlx::query_as::<_, Link>(&format!("SELECT {LINKS_SELECT_FIELDS} FROM links WHERE link_path = $1"))
        .bind(link_path)
        .fetch_optional(pool)
        .await;

    result.map_err(|err| {
        tracing::error!("Error whilst creating link err={err}");
        Error::UnknownError(Box::new(err))
    })
}
