use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

type AppState = Arc<Mutex<HashMap<String, String>>>;

#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    url: String,
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let shared_state = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/", post(shorten))
        .route("/{id}", get(redirect))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn shorten(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> impl IntoResponse {
    let id = nanoid::nanoid!(6);

    state
        .lock()
        .unwrap()
        .insert(id.clone(), payload.url.clone());

    info!("Shortened '{}' to '{}'", payload.url, id);

    let short_url = format!("http://127.0.0.1:3000/{}", id);

    (
        StatusCode::CREATED,
        Json(ShortenResponse { url: short_url }),
    )
}

async fn redirect(Path(id): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    let url_map = state.lock().unwrap();

    if let Some(url) = url_map.get(&id) {
        info!("Redirecting '{}' to '{}'", id, url);
        Redirect::permanent(url).into_response()
    } else {
        info!("ID '{}' not found", id);
        (StatusCode::NOT_FOUND, "ID not found").into_response()
    }
}
