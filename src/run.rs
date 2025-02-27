use axum::Router;
use axum::extract::FromRef;
use axum::routing::{get, get_service};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing::info;

use crate::web::handler_index;
use crate::{Config, Result};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: Arc<Config>,
}

pub async fn run(config: Config) -> Result<()> {
    let frontend_dir = config.frontend_dir.clone();
    let port = config.port;

    let state = AppState {
        config: Arc::new(config),
    };

    let routes_all = Router::new()
        .merge(routes_index(state.clone()))
        .merge(routes_static(&frontend_dir))
        .fallback(get(handler_index).with_state(state.clone()))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            ),
        );

    // Setup the server
    let ip = "127.0.0.1";
    let addr = format!("{}:{}", ip, port);
    info!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn routes_static(dir: &PathBuf) -> Router {
    let target_dir = dir.join("public");
    let dist_dir = dir.join("dist");
    Router::new()
        .nest_service(
            "/assets",
            get_service(ServeDir::new(target_dir.join("assets"))),
        )
        .nest_service("/css", get_service(ServeDir::new(target_dir.join("css"))))
        .nest_service(
            "/images",
            get_service(ServeDir::new(target_dir.join("images"))),
        )
        .nest_service("/js", get_service(ServeDir::new(dist_dir.join("js"))))
        .nest_service(
            "/manifest.json",
            get_service(ServeFile::new(target_dir.join("manifest.json"))),
        )
        .nest_service(
            "/favicon.ico",
            get_service(ServeFile::new(target_dir.join("favicon.ico"))),
        )
}

fn routes_index(state: AppState) -> Router {
    Router::new()
        .route("/", get(handler_index))
        .with_state(state)
}
