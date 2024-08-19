use poem::{web::Data, Route};
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object, OpenApi, OpenApiService};
use rand::Rng;
use sqlx::Postgres;
use std::iter;

use crate::dao::{self};

pub struct ManagementApi;

#[derive(Debug, Object, Clone, Eq, PartialEq, Default)]
struct Link {
    id: uuid::Uuid,
    url_path: String,

    title: String,
}

impl From<dao::Link> for Link {
    fn from(value: dao::Link) -> Self {
        Link {
            id: value.id,
            url_path: value.url_path,
            title: value.title,
        }
    }
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct CreateLinkInput {
    title: String,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct UpdateLinkInput {
    title: String,
}

#[derive(ApiResponse)]
enum ListLinksResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Link>>),

    #[oai(status = 500)]
    InternalServerError,
}

#[derive(ApiResponse)]
enum GetLinkResponse {
    #[oai(status = 200)]
    Ok(Json<Link>),

    #[oai(status = 404)]
    NotFound,

    #[oai(status = 500)]
    InternalServerError,
}

#[derive(ApiResponse)]
enum CreateLinkResponse {
    #[oai(status = 200)]
    Ok(Json<Link>),

    #[oai(status = 500)]
    InternalServerError,
}

#[derive(ApiResponse)]
enum UpdateLinkResponse {
    #[oai(status = 200)]
    Ok(Json<Link>),

    #[oai(status = 404)]
    NotFound,
}

#[OpenApi]
impl ManagementApi {
    /// List polylinks
    #[oai(path = "/links", method = "get")]
    async fn list_links(&self, Data(pool): Data<&sqlx::Pool<Postgres>>) -> ListLinksResponse {
        match dao::list_links(pool).await {
            Ok(links) => {
                let links: Vec<Link> = links.into_iter().map(Into::into).collect();
                ListLinksResponse::Ok(Json(links))
            },
            Err(_) => ListLinksResponse::InternalServerError,
        }
    }

    /// Get a polylink by its id
    #[oai(path = "/links/:link_id", method = "get")]
    async fn get_link(
        &self,
        Path(link_id): Path<uuid::Uuid>,
        Data(pool): Data<&sqlx::Pool<Postgres>>,
    ) -> GetLinkResponse {
        match dao::get_link(pool, &link_id).await {
            Ok(Some(link)) => GetLinkResponse::Ok(Json(link.into())),
            Ok(None) => GetLinkResponse::NotFound,
            Err(_) => GetLinkResponse::InternalServerError,
        }
    }

    /// Create a new polylink
    #[oai(path = "/links", method = "post")]
    async fn create_link(
        &self,
        input: Json<CreateLinkInput>,
        Data(pool): Data<&sqlx::Pool<Postgres>>,
    ) -> CreateLinkResponse {
        match dao::create_link(pool, &input.title).await {
            Ok(link) => CreateLinkResponse::Ok(Json(link.into())),
            Err(err) => {
                tracing::error!("Error whilst creating link err={err}");
                CreateLinkResponse::InternalServerError
            }
        }
    }

    /// Update an existing polylink
    #[oai(path = "/links/:link_id", method = "post")]
    async fn update_link(
        &self,
        link_id: Path<String>,
        input: Json<UpdateLinkInput>,
    ) -> UpdateLinkResponse {
        todo!()
    }
}

pub fn create_management_service() -> Route {
    let api_service = OpenApiService::new(ManagementApi, "PolyLink Management API", "0.0.1")
        .server("http://localhost:3000/api");

    let ui = api_service.swagger_ui();

    Route::new().nest("/", api_service).nest("/docs", ui)
}
