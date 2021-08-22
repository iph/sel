use coinbase_pro_rs::{ASync, Public, MAIN_URL};
use hyper::{service::make_service_fn, service::service_fn, Body, Request, Response, Server};
use sel::status::{get_currencies, CoinBaseClient};
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Do some startup searching, spin up background thread to do work.
    let client: Public<ASync> = Public::new_with_keep_alive(MAIN_URL, false);
    let client = CoinBaseClient { client };
    let currencies = get_currencies(&client, "USD").await.unwrap();
    for currency in currencies.iter() {
        println!("Currency: {:?}", currency);
    }
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(health_check)) });
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn health_check(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("healthy".into()))
}
