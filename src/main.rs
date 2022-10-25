use poem::{handler, listener::TcpListener, post, web::Json, Route, Server};
use serde::Deserialize;

use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;

#[derive(Debug, Deserialize)]
struct FibRequest {
    index: usize,
}

#[handler]
fn hello(req: Json<FibRequest>) -> Json<serde_json::Value> {
    Json(serde_json::json! ({
        "value": fib(req.index).to_string(),
        "index": req.index,
    }))
}

// Calculate large fibonacci numbers.
fn fib(n: usize) -> BigUint {
    if n <= 0 {
        return Zero::zero()
    }
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }
    f0
}
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new().at("/fib", post(hello));
    Server::new(TcpListener::bind("0.0.0.0:3333"))
        .run(app)
        .await
}