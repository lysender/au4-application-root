
use tera::{Tera, Context};
use std::collections::HashMap;
use serde::Serialize;
use axum::{response::{IntoResponse, Html}, extract::State};

use crate::{run::AppState, manifest::{get_lib_import_map, get_root_config_url}};
use crate::manifest::fetch_manifests;

#[derive(Serialize)]
struct IndexData {
    ga_tag_id: String,
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
    let path = format!("{}/**/*", state.config.templates_dir.display());
    let tera = Tera::new(path.as_str()).unwrap();
    let root_config_url = get_root_config_url(&state.config).expect("Unable to get root config url.");

    let portals = ImportMap {
        imports: manifests.portals,
    };
    let import_map = ImportMap {
        imports: get_lib_import_map(),
    };

    let data = IndexData {
        ga_tag_id: state.config.ga_tag_id,
        stripe_publishable_key: state.config.stripe_publishable_key,
        spa_config_url: root_config_url,
        portals,
        import_map,
        css_files: manifests.css_files,
    };
    Html(tera.render("index.html", &Context::from_serialize(&data).unwrap()).unwrap())
}
