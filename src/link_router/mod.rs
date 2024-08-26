use poem::{
    get, handler, http::StatusCode, web::{Data, Html, Path, Redirect}, FromRequest, IntoResponse, Request, RequestBody, Response, Result, Route
};
use regex::Regex;
use askama::Template;

use crate::{crawler::is_crawler, dao, AppContext};

#[derive(Debug)]
enum Platform {
    Android,
    Ios,
    Web,
    Unknown,
}

#[derive(Debug)]
enum RequestActor {
    Crawler,
    User(Platform)
}

#[derive(Template)]
#[template(path = "crawler_response.html")]
struct CrawlerResponseTemplate {
    og_title: String,
    og_description: String,
    og_type: String,
    og_url: String,
    og_image_url: String,
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

impl<'a> FromRequest<'a> for RequestActor {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let user_agent_header = req
            .headers()
            .get("User-Agent")
            .and_then(|value| value.to_str().ok());

        Ok(
            match user_agent_header {
                None => RequestActor::User(Platform::Unknown),
                Some(header_value) if is_crawler(header_value) => RequestActor::Crawler,
                Some(header_value) => RequestActor::User(get_platform_from_user_agent(header_value)),
            }
        )
    }
}

#[handler]
async fn link_handler(
    Path(link_path): Path<String>,
    request_actor: RequestActor,
    Data(app_context): Data<&AppContext>,
) -> impl IntoResponse {
    tracing::info!("Handling link link_path={link_path} request_actor={request_actor:?}");

    match dao::get_link_by_link_path(&app_context.pool, &link_path).await {
        Ok(Some(link)) => {
            match request_actor {
                RequestActor::Crawler => {
                    let response = CrawlerResponseTemplate {
                        og_title: link.title,
                        og_description: link.description,
                        og_url: link.link_path,
                        og_image_url: link.image_url,
                        og_type: "website".to_string(),
                    };
                    Html(response.render().unwrap()).into_response()
                },
                RequestActor::User(platform) => {
                    Redirect::temporary(match platform {
                        Platform::Android => link.android_destination,
                        Platform::Ios => link.ios_destination,
                        Platform::Web | Platform::Unknown => link.web_destination,
                    }).into_response()
                }
            }

        }
        Ok(None) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(format!("Link not found")),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("Unknown error")),
    }
}

pub fn create_link_router_service() -> Route {
    Route::new()
        .at("/*path", get(link_handler))
}
