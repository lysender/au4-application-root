mod config;
mod error;
mod manifest;
mod run;
mod web;

use clap::Parser;
use std::process;

use run::run;

// Re-exports
pub use config::*;
pub use error::{Error, Result};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let args = Args::parse();
    let config = Config::build(args.config.as_path()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(e) = run(config).await {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
