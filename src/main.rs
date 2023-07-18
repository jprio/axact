use std::sync::{ Arc, Mutex };

use axum::Json;
use axum::extract::ws::WebSocket;
use axum::extract::{ State, WebSocketUpgrade };
use axum::http::Response;
use axum::response::{ IntoResponse, Html };
use axum::{ Server, Router, routing::get };
use sysinfo::CpuExt;
use sysinfo::{ NetworkExt, NetworksExt, ProcessExt, System, SystemExt };
use tokio::sync::broadcast;

type Snapshot = Vec<f32>;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let (tx, _) = broadcast::channel::<Snapshot>(1);
    let app_state = AppState { tx: tx.clone() };
    let router = Router::new()
        .route("/", get(root_get))
        .route("/realtime", get(root_realtime_get))
        .route("/api/cpus", get(cpus_get))
        .route("/realtime/cpus", get(realtime_cpus_get))
        .route("/index.mjs", get(index_mjs_get))
        .route("/realtime.mjs", get(realtime_mjs_get))
        .route("/index.css", get(index_css_get))
        .with_state(app_state.clone());

    //let app_state_for_bg = app_state.clone();
    tokio::task::spawn_blocking(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            let v = sys
                .cpus()
                .iter()
                .map(|cpu| cpu.cpu_usage())
                .collect();
            tx.send(v);
            std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
        }
    });

    let server = Server::bind(&"0.0.0.0:7000".parse().unwrap()).serve(router.into_make_service());
    println!("Hello, world!");
    server.await.unwrap();
}
#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<Snapshot>,
    //cpus: Arc<Mutex<Vec<f32>>>,
}
#[axum::debug_handler]
async fn root_get() -> impl IntoResponse {
    Html(tokio::fs::read_to_string("src/index.html").await.unwrap())
}
#[axum::debug_handler]
async fn root_realtime_get() -> impl IntoResponse {
    Html(tokio::fs::read_to_string("src/realtime.html").await.unwrap())
}
#[axum::debug_handler]
async fn index_mjs_get() -> impl IntoResponse {
    let js = tokio::fs::read_to_string("src/index.mjs").await.unwrap();
    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(js)
        .unwrap()
}
#[axum::debug_handler]
async fn realtime_mjs_get() -> impl IntoResponse {
    let js = tokio::fs::read_to_string("src/realtime.mjs").await.unwrap();
    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(js)
        .unwrap()
}
#[axum::debug_handler]
async fn index_css_get() -> impl IntoResponse {
    let css = tokio::fs::read_to_string("src/index.css").await.unwrap();
    Response::builder().header("content-type", "text/css;charset=utf-8").body(css).unwrap()
}

#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let mut tx = state.tx.subscribe();
    let msg = tx.recv().await.unwrap();
    let payload = serde_json::to_string(&msg).unwrap();
    println!("{}", payload);
    payload
}
#[axum::debug_handler]
async fn realtime_cpus_get(
    State(state): State<AppState>,
    ws: WebSocketUpgrade
) -> impl IntoResponse {
    println!("ws upgraade");
    ws.on_upgrade(|ws| async { realtime_cpu_stream(state, ws).await })
}

async fn realtime_cpu_stream(app_state: AppState, mut ws: WebSocket) {
    let mut tx = app_state.tx.subscribe();
    while let Ok(msg) = tx.recv().await {
        let payload = serde_json::to_string(&msg);
        ws.send(axum::extract::ws::Message::Text(payload.unwrap())).await.unwrap();
    }
}
