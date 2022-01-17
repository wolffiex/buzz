use std::process::Command;
use warp::hyper::StatusCode;
use warp::{http::Uri, Filter, reject::Reject, reject, Rejection, Reply};


#[tokio::main]
async fn main() {
    let index = warp::get()
        .and(warp::path::end())
        .map(|| warp::redirect(Uri::from_static("/index.html")));

    let www = warp::fs::dir("./www/");
    let wasm_latest = warp::path("wasm-latest")
        .and_then(|| async move {
            match recompile_wasm() {
                Ok(uri) => {
                    Ok(warp::redirect(uri))
                }
                Err(err) => {
                    Err(reject::custom(err))
                }
            }
        })
        .recover(handle_rejection);
    //.and(warp::fs::file("target/debug/buzz_wasm.wasm"));
    // .and(recompile_wasm());
    // .and(warp::fs::file("target/debug/buzz_wasm.wasm"));


    println!("GOt her");
    warp::serve(index.or(www).or(wasm_latest))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn recompile_wasm() -> Result<Uri, CompilationError> {
    println!("Recompiling");
    let result = Command::new("rustc")
        .arg("--target").arg("wasm32-unknown-unknown")
        .arg("--out-dir").arg("./target/debug")
        .arg("-L").arg("./target/debug/deps/")
        .arg("-O")
        .arg("./src/buzz_wasm.rs")
        .output();

    let mut err_message = String::from("Compilation error");
    if let Ok(output) = result {
        if output.status.success() {
            return Ok(Uri::from_static("/wasm-foo.wasm"));
        } else {
            err_message = String::from_utf8(output.stderr).unwrap();
        }
    }
    Err(CompilationError { msg: err_message } )
}


#[derive(Debug)]
struct CompilationError {
    msg: String,
}

// We need a custom type to later extract from the `Rejection`. In
// this case, we can reuse the error type itself.
impl Reject for CompilationError {}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Rejection> {
    if let Some(e) = err.find::<CompilationError>() {
        Ok(warp::reply::with_status(format!("{}", e.msg), StatusCode::INTERNAL_SERVER_ERROR))
    } else {
        Err(err)
    }
}
