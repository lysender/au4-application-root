use axum::extract::FromRef;
use axum::routing::{get, get_service};
use axum::Router;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::info;
use tracing::Level;

use crate::config::Config;
use crate::error::Result;
use crate::web::handler_index;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: Config,
}

pub async fn run(config: Config) -> Result<()> {
    let state = AppState {
        config: config.clone(),
    };

    let routes_all = Router::new()
        .merge(routes_index(state.clone()))
        .merge(routes_static(&config.frontend_dir))
        .fallback_service(routes_fallback(state))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            ),
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
    let target_dir = dir.join("public");
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
        .nest_service("/js", get_service(ServeDir::new(target_dir.join("js"))))
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

fn routes_fallback(state: AppState) -> Router {
    // Catch all request that don't match the static files
    // and other routes
    Router::new().nest_service("/", get(handler_index).with_state(state))
}
