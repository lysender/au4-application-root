use askama::Template;
use axum::http::HeaderMap;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use serde::Serialize;
use std::collections::HashMap;

use crate::manifest::fetch_manifests;
use crate::{
    manifest::{get_lib_import_map, get_root_config_url},
    run::AppState,
};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexData {
    ga_tag_id: Option<String>,
    stripe_publishable_key: String,
    spa_config_url: String,
    portals: ImportMap,
    import_map: ImportMap,
    css_files: Vec<String>,
}

#[derive(Serialize)]
struct ImportMap {
    imports: HashMap<String, String>,
}

pub async fn handler_index(State(state): State<AppState>) -> impl IntoResponse {
    let manifests = fetch_manifests(&state.config).await.unwrap();
    let root_config_url =
        get_root_config_url(&state.config).expect("Unable to get root config url.");

    let portals = ImportMap {
        imports: manifests.portals,
    };
    let import_map = ImportMap {
        imports: get_lib_import_map(),
    };

    let tpl = IndexData {
        ga_tag_id: state.config.ga_tag_id,
        stripe_publishable_key: state.config.stripe_publishable_key,
        spa_config_url: root_config_url,
        portals,
        import_map,
        css_files: manifests.css_files,
    };

    let mut headers = HeaderMap::new();
    headers.insert("Surrogate-Control", "no-store".parse().unwrap());
    headers.insert(
        "Cache-Control",
        "no-store, no-cache, must-revalidate, proxy-revalidate"
            .parse()
            .unwrap(),
    );
    headers.insert("Pragma", "no-cache".parse().unwrap());
    headers.insert("Expires", "0".parse().unwrap());

    (headers, Html(tpl.render().unwrap()))
}
