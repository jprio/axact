use std::sync::{ Arc, Mutex };

use axum::Json;
use axum::extract::State;
use axum::http::Response;
use axum::response::{ IntoResponse, Html };
use axum::{ Server, Router, routing::get };
use sysinfo::CpuExt;
use sysinfo::{ NetworkExt, NetworksExt, ProcessExt, System, SystemExt };
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let router = Router::new()
        .route("/", get(root_get))
        .route("/api/cpus", get(cpus_get))
        .route("/index.js", get(index_js_get))
        .with_state(AppState { sys: Arc::new(Mutex::new(System::new_all())) });

    let server = Server::bind(&"0.0.0.0:7000".parse().unwrap()).serve(router.into_make_service());
    println!("Hello, world!");
    server.await.unwrap();
}
#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}
#[axum::debug_handler]
async fn root_get() -> impl IntoResponse {
    Html(tokio::fs::read_to_string("src/index.html").await.unwrap())
}
#[axum::debug_handler]
async fn index_js_get() -> impl IntoResponse {
    let js = tokio::fs::read_to_string("src/index.js").await.unwrap();
    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(js)
        .unwrap()
}

async fn sysinfo_get(State(state): State<AppState>) -> String {
    use std::fmt::Write;
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    let cpus = sys.cpus();
    let mut s = String::new();
    println!("n of cpu {}", cpus.len());
    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // Display system information:
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());

    for (i, cpu) in cpus.iter().enumerate() {
        writeln!(&mut s, "cpu usage {i} : {}", cpu.cpu_usage()).unwrap();
    }
    println!("{}", s);
    s
}

#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    let v: Vec<_> = sys
        .cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .collect();
    Json(v)
}
