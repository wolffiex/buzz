use axum::{response::Html, routing::get, Router};
use buzz;
use std::net::SocketAddr;
use std::process::Command;

#[tokio::main]
async fn main() {
    const TEMP_DIR: &str = "FDLKJ.wasm";
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/wasm", get(move || wasm_handler(TEMP_DIR)));

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

async fn wasm_handler(out_dir: &str) -> Html<String> {
    let result = Command::new("cargo")
        .arg("build")
        .arg("--package").arg("wasm")
        .arg("--target").arg("wasm32-unknown-unknown")
        .arg("--target-dir").arg(out_dir)
        .output();
    Html(format!("Hello wasm {:?}", result).into())
}
