use std::convert::Infallible;
use std::process::{Stdio, Command};
use std::io;
use warp::hyper::StatusCode;
use warp::{http::Uri, Filter, reject, Rejection, Reply};

#[tokio::main]
async fn main() {
    let index = warp::get()
        .and(warp::path::end())
        .map(|| warp::redirect(Uri::from_static("/index.html")));

    let www = warp::fs::dir("./www/");
    let wasm = warp::path("wasm")
        .and_then(recompile_wasm)
        //.and(warp::fs::file("target/debug/buzz_wasm.wasm"))
        .recover(handle_rejection);



    println!("GOt her");
    warp::serve(index.or(www).or(wasm))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn recompile_wasm() -> Result<impl warp::Reply, warp::Rejection> {
    println!("Recompiling");
    let result = Command::new("rustc")
        .arg("--target").arg("wasm32-unknown-unknown")
        .arg("--out-dir").arg("./target/debug")
        .arg("-L").arg("./target/debug/deps/")
        .arg("-O")
        .arg("./src/buzz_wasm.rs")
        // .arg("./src/simwasm.rs")
        .stderr(Stdio::piped())
        .output();

    let mut err_message = String::from("Compilation error");
    if let Ok(output) = result {
        if output.status.success() {
            return Ok(warp::reply::reply())
        } else {
            err_message = String::from_utf8(output.stderr).unwrap();
        }
    }

    print!("{}", err_message);
    println!();
    return Err(warp::reject::reject());

}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = (
            StatusCode::INTERNAL_SERVER_ERROR,
            "fRUME Inal Server Error".to_string(),
        );

    Ok(warp::reply::with_status(message, code))
}
