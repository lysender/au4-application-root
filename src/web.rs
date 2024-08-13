use askama::Template;
use axum::body::Body;
use axum::{extract::State, response::Response};
use serde::Serialize;
use std::collections::HashMap;

use crate::manifest::fetch_manifests;
use crate::Result;
use crate::{
    manifest::{get_lib_import_map, get_root_config_url},
    run::AppState,
};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexData {
    ga_tag_id: Option<String>,
    spa_config_url: String,
    portals: ImportMap,
    import_map: ImportMap,
    css_files: Vec<String>,
}

#[derive(Serialize)]
struct ImportMap {
    imports: HashMap<String, String>,
}

pub async fn handler_index(State(state): State<AppState>) -> Result<Response<Body>> {
    let config = state.config.clone();
    let manifests = fetch_manifests(&config).await?;
    let root_config = get_root_config_url(&config)?;

    let portals = ImportMap {
        imports: manifests.portals,
    };
    let import_map = ImportMap {
        imports: get_lib_import_map(),
    };

    let tpl = IndexData {
        ga_tag_id: config.ga_tag_id.clone(),
        spa_config_url: root_config.url,
        portals,
        import_map,
        css_files: manifests.css_files,
    };

    let res = Response::builder()
        .status(200)
        .header("Surrogate-Control", "no-store")
        .header(
            "Cache-Control",
            "no-store, no-cache, must-revalidate, proxy-revalidate",
        )
        .header("Pragma", "no-cache")
        .header("Expires", "0")
        .body(Body::from(tpl.render().unwrap()))
        .unwrap();

    Ok(res)
}
