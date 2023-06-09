// #![deny(clippy::all)]
// #![deny(clippy::pedantic)]
// #![deny(clippy::nursery)]
// #![deny(clippy::cargo)]
// #![deny(missing_docs)]

use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use axum::{
    body::Bytes,
    extract::{Path, State},
    http::Response,
    routing::get,
    Router,
};

type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
pub struct AppState {
    _db: HashMap<String, Bytes>,
}

pub async fn get_state() -> Arc<RwLock<AppState>> {
  SharedState::default()
}

pub fn get_server(state: SharedState) -> axum::Server<hyper::server::conn::AddrIncoming, axum::routing::IntoMakeService<axum::Router>> {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/:name", get(greet))
        .with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr).serve(app.into_make_service())
}

pub async fn health_check() -> Response<String> {
    Response::builder()
        .status(200)
        .body("ok".to_string())
        .unwrap()
}

pub async fn greet(Path(key): Path<String>, State(_state): State<SharedState>) -> String {
    format!("Hello {}!", &key)
}
