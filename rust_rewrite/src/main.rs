// #![allow(unused)] // Silence warnings for dev purposes

mod api;
mod auth;
mod db;
mod error;
mod models;
mod repository;

use std::sync::Arc;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

use tracing::info;
use tracing_subscriber;

pub use self::error::{Error, Result};
use crate::db::create_db_connection;
use crate::repository::PageRepository;
use axum::Router;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

//Metrics
use metrics_exporter_prometheus::PrometheusBuilder;



/// to access the interactive OpenAPI documentation, go to localhost:8080/swagger-ui
/// to access the OpenAPI JSON, go to localhost:8080/api-doc/openapi.json

// this: #[], called Attribute, is basically like annotations in Java
// this generates OpenAPI documentation for the paths specified? i think
// so if we want to add more paths, we just do #[openapi(paths(path1, path2, path3))]
#[derive(OpenApi)] // this attribute derives the OpenApi impl for the struct
#[openapi(paths(
    api::api_search,
    api::api_login,
    api::api_register,
    api::api_logout,
    api::api_weather,
    api::root_dummy,
    api::register_dummy,
    api::login_dummy
))] // this attribute specifies the paths that will be documented
    // structs are like classes in Java, but without methods
struct ApiDoc; // this is the struct that will be used to generate the OpenAPI documentation

/* fn setup_metrics_recorder() -> PrometheusHandle {
    PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to install Prometheus recorder")
} */

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        //.with_current_span(true)
        //.with_span_list(true)
        .with_target(true)
        .with_level(true)
        //.with_thread_names(true)
        //.with_thread_ids(true)
        .flatten_event(true)
        .init();

    // sets server to listen on localhost:8084
    let listener = TcpListener::bind("0.0.0.0:8084").await.unwrap();
    info!("LISTENING on {:?}", listener.local_addr());

    let db = create_db_connection().await.unwrap(); // create new database connection
    let db = Arc::new(db); // to manage shared state
    let open_api_doc = ApiDoc::openapi();

    let repo = Arc::new(PageRepository::new(db.as_ref().clone()).await); // Create PageRepository

    //Setup metrics
    let prometheus_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to install Prometheus recorder");

    let app = Router::new()
        .nest("/api/", api::routes(db.clone(), repo.clone(), prometheus_handle.clone())) // merge the routes from api.rs
        .merge(SwaggerUi::new("/doc/swagger-ui").url("/doc/api-doc/openapi.json", open_api_doc)) // add swagger ui, and openapi doc
        .layer(CookieManagerLayer::new())
        .layer(CorsLayer::new().allow_credentials(true))
        .route("/metrics", axum::routing::get(move || async move {
            prometheus_handle.render()
        }));

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

}
