use actix_web::{
    get,
    http::{uri::Parts, Uri},
    web::Redirect,
    App, HttpRequest, HttpServer, Responder,
};
use regex::Regex;

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

#[get("{tail:.*}")]
async fn handler(req: HttpRequest) -> impl Responder {
    let user_agent = req.headers().get("User-Agent").unwrap().to_str().unwrap();
    let platform = get_platform_from_user_agent(user_agent);
    let target_uri = match platform {
        Platform::Android => {
            // TODO We should actually redirect to the play store here
            // as the app on android should natively handle these thinks
            let mut parts = Parts::default();
            parts.scheme = Some("nowu".parse().unwrap());
            parts.authority = Some("app".parse().unwrap());
            parts.path_and_query = req.uri().path_and_query().cloned();

            Uri::from_parts(parts).unwrap()
        }
        Platform::Web => Uri::from_static("https://google.com"),
        _ => req.uri().clone(),
    };

    Redirect::to(target_uri.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(handler))
        .bind(("192.168.1.11", 8000))?
        .run()
        .await
}
