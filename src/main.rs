use axum::body::Body;
use axum::handler::Handler;
use axum::{
    body::{BoxBody, StreamBody},
    http::{header, request::Request, HeaderMap, HeaderValue},
    response::IntoResponse,
    response::{Html, Response},
    routing::get,
    Router,
};
use std::collections::HashMap;
use std::env::temp_dir;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Command;
use std::str;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_util::io::ReaderStream;
use tower::ServiceExt;
use tower_http::services::fs::ServeDir;
use uuid::Uuid;

use serde::Deserialize;
use serde_json::{Result, Value};

#[derive(Deserialize, Debug)]
#[serde(tag = "reason")]
enum CargoMessage {
    #[serde(rename = "compiler-message")]
    CompilerMessage { package_id: String, message: Value },
    #[serde(rename = "build-finished")]
    BuildFinished { success: bool },
}

#[tokio::main]
async fn main() {
    let mut temp_dir = temp_dir();
    temp_dir.push(format!("buzz-{}", Uuid::new_v4()));
    println!("Build dir: {}", temp_dir.to_str().unwrap());
    let wasm_build_dir = Arc::new(Mutex::new(temp_dir));

    let app = Router::new()
        .route(
            "/wasm",
            get(move || wasm_handler(Arc::clone(&wasm_build_dir))),
        )
        .fallback(www_handler.into_service());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn www_handler(req: Request<Body>) -> impl IntoResponse {
    return match ServeDir::new("www").oneshot(req).await {
        Ok(res) => res,
        Err(_) => unimplemented!(),
    };
}

async fn wasm_handler(dir_lock: Arc<Mutex<PathBuf>>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    let out_dir = dir_lock.lock().await;
    let dir_name = out_dir.to_str().unwrap();
    #[rustfmt::skip]
    let result = Command::new("cargo")
        .arg("build")
        .arg("--package").arg("wasm")
        .arg("--target").arg("wasm32-unknown-unknown")
        .arg("--target-dir").arg(dir_name)
        .arg("--message-format").arg("json")
        .output();

    let body: Response<BoxBody> = match result {
        Ok(output) => {
            let json_lines = str::from_utf8(&output.stdout).unwrap();

            let messages = json_lines.lines().map(serde_json::from_str);

            let (build_result, compiler_messages): (Option<bool>, Vec<Value>) = messages.fold(
                (None, Vec::new()),
                |mut last, cargo_message| match cargo_message.expect("Missing cargo message type") {
                    CargoMessage::CompilerMessage { message, .. } => {
                        last.1.push(message);
                        (last.0, last.1)
                    }
                    CargoMessage::BuildFinished { success } => (Some(success), last.1),
                },
            );

            if matches!(build_result, Some(true)) {
                println!(
                    "Build result : {}",
                    build_result.expect("Compiler produces build-result")
                );
                let mut wasm_file = out_dir.clone();
                wasm_file.push("wasm32-unknown-unknown/debug/wasm.wasm");

                match tokio::fs::File::open(wasm_file).await {
                    Ok(file) => {
                        headers.insert(
                            header::CONTENT_TYPE,
                            HeaderValue::from_static("application/wasm"),
                        );
                        let stream = ReaderStream::new(file);
                        StreamBody::new(stream).into_response()
                    }
                    Err(_) => ().into_response(),
                }
            } else {
                println!(
                    "Build result : {}",
                    build_result.expect("Compiler produces build-result")
                );
                for v in compiler_messages {
                    println!("cm: {:#?}", v);
                }
                ().into_response()
            }
        }
        Err(_) => ().into_response(),
    };

    (headers, body)
    // Html(format!("Hello wasm {:?}", result).into())
}
