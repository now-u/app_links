use poem::{get, handler, web::{Data, Path}, FromRequest, Request, RequestBody, Result, Route};
use regex::Regex;

use crate::dao;

#[derive(Debug)]
enum Platform {
    Android,
    Ios,
    Web,
    Unknown,
}

fn get_platform_from_user_agent(user_agent: &str) -> Platform {
    println!("User agent: {}", user_agent);
    if Regex::new(r"Android").unwrap().is_match(user_agent) {
        return Platform::Android;
    }
    if Regex::new(r"iPhone|iPad|iPod")
        .unwrap()
        .is_match(user_agent)
    {
        return Platform::Ios;
    }
    if Regex::new(r"Windows|Macintosh|Linux")
        .unwrap()
        .is_match(user_agent)
    {
        return Platform::Web;
    }
    return Platform::Unknown;
}

impl<'a> FromRequest<'a> for Platform {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let user_agent_header = req
            .headers()
            .get("User-Agent")
            .and_then(|value| value.to_str().ok());

        Ok(match user_agent_header {
            None => Platform::Unknown,
            Some(header_value) => get_platform_from_user_agent(header_value),
        })
    }
}

#[handler]
async fn link_handler(
    Path(link_path): Path<String>,
    platform: Platform,
    Data(pool): Data<&sqlx::Pool<sqlx::Postgres>>,
) -> String {
    tracing::info!("Handling link link_path={link_path}");

    match dao::get_link_by_url_path(pool, &link_path).await {
        Ok(Some(link)) => format!("hello {link_path} on {:?}. Link title={}", platform, link.title),
        Ok(None) => format!("Link not found"),
        Err(_) => format!("Unknown error"),
    }
}

pub fn create_link_router_service() -> Route {
    Route::new().at("/:link_path", get(link_handler))
}
