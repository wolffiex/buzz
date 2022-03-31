use axum::{
    body::{Body, BoxBody, HttpBody, StreamBody},
    http::{header, StatusCode, HeaderMap},
    response::Response,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use buzz;
use std::env::temp_dir;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Command;
use std::str;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_util::io::ReaderStream;
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

async fn wasm_handler(dir_lock: Arc<Mutex<PathBuf>>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

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

    let body : Response<BoxBody> = match result {
        Ok(output) => {
            let stdo = output.stdout;
            let json = str::from_utf8(&stdo);
            println!("r: {}", json.unwrap());

            let mut wasm_file = out_dir.clone();
            wasm_file.push("wasm32-unknown-unknown/debug/wasm.wasm");

            let file = tokio::fs::File::open(wasm_file).await.unwrap();
            // {
            //     Ok(file) => file,
            //     Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
            // };
            // convert the `AsyncRead` into a `Stream`
            let stream = ReaderStream::new(file);
            // convert the `Stream` into an `axum::body::HttpBody`
            StreamBody::new(stream).into_response()
        }
        Err(e) => ().into_response(),
    };
    ([(header::CONTENT_TYPE, "application/wasm")]);

    (headers, body)
    // Html(format!("Hello wasm {:?}", result).into())
}
