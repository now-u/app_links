use poem::{error::InternalServerError, web::Data, Request, Result, Route};
use poem_openapi::{
    auth::ApiKey, param::Path, payload::Json, ApiResponse, Object, OpenApi, OpenApiService,
    SecurityScheme,
};
use url::Url;

use crate::{dao::{self, update_link, LinkData}, AppContext};

pub struct ManagementApi;

#[derive(Debug, Object, Clone, Eq, PartialEq, Default)]
struct Link {
    id: uuid::Uuid,
    link_path: String,
    link_url: String,

    title: String,
    description: String,
    image_url: String,
}

impl dao::Link {
    fn serialize(self, base_url: &Url) -> Link {
        Link {
            id: self.id,
            link_url: base_url.join(&self.link_path).unwrap().to_string(),
            link_path: self.link_path,
            title: self.title,
            description: self.description,
            image_url: self.image_url,
        }
    }

    fn to_json(self, base_url: &Url) -> Json<Link> {
        Json(self.serialize(base_url))
    }
}

#[derive(ApiResponse)]
enum ListLinksResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Link>>),
}

#[derive(ApiResponse)]
enum GetLinkResponse {
    #[oai(status = 200)]
    Ok(Json<Link>),

    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
enum CreateLinkResponse {
    #[oai(status = 200)]
    Ok(Json<Link>),
}

#[derive(ApiResponse)]
enum UpdateLinkResponse {
    #[oai(status = 200)]
    Ok(Json<Link>),

    #[oai(status = 404)]
    NotFound,
}

#[derive(SecurityScheme)]
#[oai(
    ty = "api_key",
    key_name = "X-API-Key",
    key_in = "header",
    checker = "api_key_checker"
)]
struct ApiKeyAuth(());

async fn api_key_checker(
    request: &Request,
    api_key: ApiKey,
) -> Option<()> {
    let app_context: &AppContext = request.data().unwrap();
    if api_key.key == app_context.api_key {
        return Some(());
    }
    return None;
}

impl From<dao::Error> for poem::Error {
    fn from(value: dao::Error) -> Self {
        InternalServerError(value)
    }
}

#[OpenApi]
impl ManagementApi {
    /// List polylinks
    #[oai(path = "/links", method = "get")]
    async fn list_links(
        &self,
        Data(app_context): Data<&AppContext>,
        _auth: ApiKeyAuth,
    ) -> Result<ListLinksResponse> {
        let links = dao::list_links(&app_context.pool).await?;
        let links: Vec<Link> = links.into_iter().map(|item| item.serialize(&app_context.base_url)).collect();
        Ok(ListLinksResponse::Ok(Json(links)))
    }

    /// Get a polylink by its id
    #[oai(path = "/links/:link_id", method = "get")]
    async fn get_link(
        &self,
        Path(link_id): Path<uuid::Uuid>,
        Data(app_context): Data<&AppContext>,
        _auth: ApiKeyAuth,
    ) -> Result<GetLinkResponse> {
        Ok(match dao::get_link(&app_context.pool, &link_id).await? {
            Some(link) => GetLinkResponse::Ok(link.to_json(&app_context.base_url)),
            None => GetLinkResponse::NotFound,
        })
    }

    /// Create a new polylink
    #[oai(path = "/links", method = "post")]
    async fn create_link(
        &self,
        Json(input): Json<LinkData>,
        Data(app_context): Data<&AppContext>,
        _auth: ApiKeyAuth,
    ) -> Result<CreateLinkResponse> {
        let link = dao::create_link(&app_context.pool, input).await?;

        Ok(CreateLinkResponse::Ok(link.to_json(&app_context.base_url)))
    }

    /// Update an existing polylink
    #[oai(path = "/links/:link_id", method = "post")]
    async fn update_link(
        &self,
        Path(link_id): Path<uuid::Uuid>,
        Json(input): Json<LinkData>,
        Data(app_context): Data<&AppContext>,
        _auth: ApiKeyAuth,
    ) -> Result<UpdateLinkResponse> {
        Ok(match update_link(&app_context.pool, &link_id, input).await? {
            Some(link) => UpdateLinkResponse::Ok(link.to_json(&app_context.base_url)),
            None => UpdateLinkResponse::NotFound,
        })
    }
}

pub fn create_management_service(base_url: &Url) -> Route {
    let api_service = OpenApiService::new(ManagementApi, "PolyLink Management API", "0.0.1")
        .server(base_url.join("/api").expect("Cannot join base url with api").to_string());

    let ui = api_service.swagger_ui();
    let json_spec_endpoint = api_service.spec_endpoint();
    let yaml_spec_endpoint = api_service.spec_endpoint_yaml();

    Route::new().nest("/", api_service)
        .nest("/docs", ui)
        .nest("/docs/spec.json", json_spec_endpoint)
        .nest("/docs/spec.yaml", yaml_spec_endpoint)
}
