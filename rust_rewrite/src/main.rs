#![allow(unused)]  // Silence warnings for dev purposes

mod error;
mod models;
mod api;
mod auth;
use api::{api_search, api_login, api_register, api_logout};
use tower_cookies::{CookieManager, CookieManagerLayer};

pub use self::error::{Error, Result};

use tokio::net::TcpListener;

use axum::response::{Html, Response};
use axum::routing::{get, post};
use axum::{middleware, Router};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


/// to access the interactive OpenAPI documentation, go to localhost:8080/swagger-ui
/// to access the OpenAPI JSON, go to localhost:8080/api-doc/openapi.json

// this: #[], called Attribute, is basically like annotations in Java
// this generates OpenAPI documentation for the paths specified? i think
// so if we want to add more paths, we just do #[openapi(paths(path1, path2, path3))]
#[derive(OpenApi)] // this attribute derives the OpenApi impl for the struct
#[openapi(paths(hello, api::api_search, api::api_login, api::api_register, api::api_logout))] // this attribute specifies the paths that will be documented
// structs are like classes in Java, but without methods
struct ApiDoc; // this is the struct that will be used to generate the OpenAPI documentation


// this is the function that will be called when the path is hit
// here we define the path, the response status, the response description, and the response body
// which will be put into the OpenAPI documentation
#[utoipa::path(get,
   path = "/hello", responses(
    (status = 200, description = "Hello Sauron!", body = String),
  )
)]
async fn hello() -> Html<&'static str> {
  Html("<h1>Hello Sauron!</h1>")
}

#[tokio::main]
async fn main() {
  // sets server to listen on localhost:8080
	let listener = TcpListener::bind("localhost:8080").await.unwrap();
	println!("->> LISTENING on {:?}\n", listener.local_addr());


  async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
  }

  let app = Router::new()
  .route("/hello", get(hello))
  .merge(api::routes()) // merge the routes from api.rs
  .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi())) // add swagger ui, and openapi doc
  .layer(CookieManagerLayer::new())
  .layer(middleware::map_response(main_response_mapper));

  // start server 
  axum::serve(listener, app.into_make_service())
		.await
		.unwrap();
}
