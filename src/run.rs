
use std::path::PathBuf;
use std::process;
use axum::http::Method;
use axum::Router;
use axum::routing::get_service;
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::ServeDir;
use tower::ServiceBuilder;
use tracing::Level;
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use tracing::{info, error};

use crate::error::Result;
use crate::config::Config;

pub async fn run(config: &Config) -> Result<()> {
    let routes_all = Router::new()
        .merge(routes_static(&config.public_dir))
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
    Router::new().nest_service("/", get_service(ServeDir::new(dir)))
}
