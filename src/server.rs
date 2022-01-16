use std::process::Command;
use warp::{http::Uri, Filter};

#[tokio::main]
async fn main() {
    let index = warp::get()
        .and(warp::path::end())
        .map(|| warp::redirect(Uri::from_static("/index.html")));

    let www = warp::fs::dir("./www/");
    let wasm = warp::path("wasm")
        .map(|| recompile_wasm())
        .untuple_one()
        .and(warp::fs::file("target/debug/buzz_wasm.wasm"));

    println!("GOt her");
    warp::serve(index.or(www).or(wasm))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn recompile_wasm() {
    println!("Recompiling");
    Command::new("rustc")
        .arg("--target").arg("wasm32-unknown-unknown")
        .arg("--out-dir").arg("./target/debug")
        .arg("-O")
        .arg("./src/buzz_wasm.rs")
        .spawn()
        .expect("Wasm library build failed");

    println!("Did recom");
}
