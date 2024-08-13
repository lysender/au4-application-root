use crate::{Config, Error, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct AssetManifest {
    pub files: HashMap<String, String>,
}

#[derive(Deserialize, Serialize)]
pub struct AssetImport {
    pub portals: HashMap<String, String>,
    pub css_files: Vec<String>,
}

#[derive(Deserialize)]
pub struct RootConfig {
    pub url: String,
}

#[derive(Deserialize)]
pub struct RootConfigV2 {
    #[serde(rename = "src/root-config-legacy.js")]
    pub entry: AssetEntry,
}

#[derive(Deserialize)]
pub struct AssetEntry {
    pub file: String,
    pub name: String,
    pub src: String,

    #[serde(rename = "isEntry")]
    pub is_entry: bool,
}

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36";
const JSON_CONTENT_TYPE: &str = "application/json";

const LIB_IMPORT_MAP: [(&'static str, &'static str); 16] = [
    (
        "React",
        "/assets/root/js/vendors/react/18.3.1/umd/react.production.min.js",
    ),
    (
        "react",
        "/assets/root/js/vendors/react/18.3.1/umd/react.production.min.js",
    ),
    (
        "react-dom",
        "/assets/root/js/vendors/react-dom/18.3.1/umd/react-dom.production.min.js",
    ),
    (
        "react-dom/server",
        "/assets/root/js/vendors/react-dom/18.3.1/umd/react-dom-server.browser.production.min.js",
    ),
    (
        "single-spa",
        "/assets/root/js/vendors/single-spa/6.0.1/system/single-spa.min.js",
    ),
    (
        "lodash",
        "/assets/root/js/vendors/lodash/4.17.21/lodash.min.js",
    ),
    ("axios", "/assets/root/js/vendors/axios/0.28.1/axios.min.js"),
    ("antd", "/assets/root/js/vendors/antd/5.19.3/antd.min.js"),
    (
        "immutable",
        "/assets/root/js/vendors/immutable/3.7.6/immutable.min.js",
    ),
    (
        "@ant-design/icons",
        "/assets/root/js/vendors/ant-design-icons/5.4.0/index.umd.min.js",
    ),
    (
        "react-virtualized",
        "/assets/root/js/vendors/react-virtualized/9.22.3/react-virtualized.min.js",
    ),
    (
        "react-beautiful-dnd",
        "/assets/root/js/vendors/react-beautiful-dnd/13.1.0/react-beautiful-dnd.min.js",
    ),
    (
        "react-query",
        "/assets/root/js/vendors/react-query/3.39.3/react-query.production.js",
    ),
    (
        "dayjs",
        "/assets/root/js/vendors/dayjs/1.11.12/dayjs.min.js",
    ),
    ("luxon", "/assets/root/js/vendors/luxon/3.4.4/dayjs.min.js"),
    (
        "moment",
        "/assets/root/js/vendors/moment/2.29.1/moment.min.js",
    ),
];

pub fn get_lib_import_map() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for (name, url) in LIB_IMPORT_MAP.iter() {
        map.insert(String::from(*name), String::from(*url));
    }
    map
}

pub fn get_root_config_url(config: &Config) -> Result<RootConfig> {
    let filename = config.frontend_dir.join("spa-config.json");
    match fs::read_to_string(filename) {
        Ok(json) => match serde_json::from_str(json.as_str()) {
            Ok(root) => Ok(root),
            Err(err) => Err(Error::RootConfigError(format!(
                "Unable to parse root config file - {}",
                err
            ))),
        },
        Err(err) => Err(Error::RootConfigError(format!(
            "Unable to read root config file - {}",
            err
        ))),
    }
}

pub async fn fetch_manifests(config: &Config) -> Result<AssetImport> {
    let mut portals: HashMap<String, String> = HashMap::new();
    let mut css_files: Vec<String> = Vec::new();

    let targets: Vec<(&str, &str)> = vec![
        ("apm", config.apm_manifest_url.as_str()),
        ("notifications", config.notifications_manifest_url.as_str()),
        ("comments", config.comments_manifest_url.as_str()),
        ("admin", config.admin_manifest_url.as_str()),
    ];

    for (name, url) in targets.iter() {
        let manifest = fetch_manifest(name, url).await?;
        if let Some(js) = manifest.files.get("main.js") {
            let portal_name: String = format!("@portal/{}", name);
            portals.insert(portal_name, js.clone());
        }
        if let Some(css) = manifest.files.get("main.css") {
            css_files.push(css.clone());
        }
    }

    Ok(AssetImport { portals, css_files })
}

async fn fetch_manifest(name: &str, url: &str) -> Result<AssetManifest> {
    let result = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .send()
        .await;

    match result {
        Ok(response) => {
            if response.status().is_success() {
                match response.json().await {
                    Ok(manifest) => Ok(manifest),
                    Err(err) => Err(Error::ManifestError(format!(
                        "Unable to parse asset manifest for {}. Error: {}",
                        name, err
                    ))),
                }
            } else {
                Err(Error::ManifestError(format!(
                    "Unable to fetch asset manifest for {}. Error: {}",
                    name,
                    response.status()
                )))
            }
        }
        Err(err) => Err(Error::ManifestError(format!(
            "Unable to fetch asset manifest for {}. Error: {}",
            name, err
        ))),
    }
}
