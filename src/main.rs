use axum::{response::Html, routing::get, Router};
use buzz;
use std::env::temp_dir;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut temp_dir = temp_dir();
    temp_dir.push(format!("buzz-{}", Uuid::new_v4()));
    println!("Build dir: {}", temp_dir.to_str().unwrap());
    let shared_dir = Arc::new(Mutex::new(temp_dir));

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/wasm", get(move || wasm_handler(Arc::clone(&shared_dir))));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<String> {
    let id = buzz::get_id();
    Html(format!("Hello buzz {:?}", id).into())
}

async fn wasm_handler(dir_lock: Arc<Mutex<PathBuf>>) -> Html<String> {
    let out_dir = dir_lock.lock().await;
    let dir_name = out_dir.to_str().unwrap();
    println!("drn {}", dir_name);
    #[rustfmt::skip]
    let result = Command::new("cargo")
        .arg("build")
        .arg("--package").arg("wasm")
        .arg("--target").arg("wasm32-unknown-unknown")
        .arg("--target-dir").arg(dir_name)
        .arg("--message-format").arg("json")
        .output();
    Html(format!("Hello wasm {:?}", result).into())
}
