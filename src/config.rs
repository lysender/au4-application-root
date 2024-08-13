use clap::Parser;
use serde::Deserialize;
use std::path::Path;
use std::{fs, path::PathBuf};

use crate::Result;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub port: u16,
    pub frontend_dir: PathBuf,
    pub manifest_cache: bool,
    pub apm_manifest_url: String,
    pub notifications_manifest_url: String,
    pub comments_manifest_url: String,
    pub admin_manifest_url: String,
    pub ga_tag_id: Option<String>,
    pub stripe_publishable_key: String,
    pub new_relic_app_name: String,
    pub new_relic_license_key: Option<String>,
}

impl Config {
    pub fn build(filename: &Path) -> Result<Config> {
        let toml_string = match fs::read_to_string(filename) {
            Ok(str) => str,
            Err(_) => {
                return Err("Unable to read config file.".into());
            }
        };

        let config: Config = match toml::from_str(toml_string.as_str()) {
            Ok(value) => value,
            Err(err) => {
                println!("{:?}", err);
                return Err("Unable to parse config file.".into());
            }
        };

        let frontend_dir = Path::new(&config.frontend_dir);
        if !frontend_dir.exists() {
            return Err("Frontend dir does not exists.".into());
        }

        Ok(config)
    }
}

/// Web server to serve react micro-frontends
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// TOML configuration file
    #[arg(short, long, value_name = "FILE.toml")]
    pub config: PathBuf,
}
