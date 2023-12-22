
use std::path::{PathBuf, Path};
use axum::http::Method;
use axum::Router;
use axum::response::{IntoResponse, Html};
use axum::routing::{get_service, get};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use tower::ServiceBuilder;
use tracing::Level;
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use tracing::{info, error};

use crate::error::Result;
use crate::config::Config;

pub async fn run(config: &Config) -> Result<()> {
    let routes_all = Router::new()
        .merge(routes_index())
        .merge(routes_static(&config.public_dir))
        .fallback_service(routes_fallback())
        .layer(ServiceBuilder::new()
            .layer(TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
            )
        );

    // Setup the server
    let ip = "127.0.0.1";
    let addr = format!("{}:{}", ip, config.port);
    info!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn routes_static(dir: &PathBuf) -> Router {
    Router::new()
        .nest_service("/assets", get_service(ServeDir::new(dir.join("assets"))))
        .nest_service("/css", get_service(ServeDir::new(dir.join("css"))))
        .nest_service("/images", get_service(ServeDir::new(dir.join("images"))))
        .nest_service("/js", get_service(ServeDir::new(dir.join("js"))))
}

fn routes_index() -> Router {
    Router::new()
        .route("/", get(handler_index))
}

fn routes_fallback() -> Router {
    Router::new()
        .nest_service("/", get(handler_index))
}

async fn handler_index() -> impl IntoResponse {
    info!("this is from index...");

    Html("This is the index page...")
}
