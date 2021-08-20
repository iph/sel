use coinbase_pro_rs::{ASync, Public, MAIN_URL};
use hyper::{service::make_service_fn, service::service_fn, Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Instant;
use std::fmt::Debug;
use coinbase_pro_rs::structs::public::Product;
use std::error::Error;

#[derive(PartialEq, Clone, Debug)]
struct ScannedProduct {
    id: String,
    name: String,
    base_currency: String,
    quote_currency: String,
}

// ExecutionStatus is "where in the world is the stored fill?"
// When a user enters an amount, the first state is always "Waiting"
// When the enter amount is hit, it will go from waiting -> pending-fill
// as there is an intermediate state of executing the order that's worth tracking.
//
// Afterwards, the order is fulfilled and the state is "executed". Finally, there is a point
// where we have achieved the exit criteria (loss or gain) and need to leave. First we enter
// pending-exit, execute orders to leave, and then finally exit.
//
// The remaining statuses are Expired (time to achieve ended) or Error. Error can only be achieved
// from PendingFill, as it usually means there are inadequate funds and it can't be executed on.
enum ExecutionStatus {
    // Waiting is within the time of execution,
    Waiting,
    PendingFill,
    Executed,
    PendingExit,
    Exited,
    Expired,
    Error
}

trait ProvideProduct {
    fn get_products(&self) -> Result<Vec<Product>, dyn Error>;
}

impl Default for ExecutionStatus {
    fn default() -> Self {
       ExecutionStatus::Waiting
    }
}

#[derive(PartialEq, Clone, Debug)]
struct StoredBet {
    id: String,
    end_date: Instant,
    enter_amount: f64,
    stop_loss: f64,
    stop_gain: f64,
    status: ExecutionStatus
}

struct CoinBaseClient {
    client: Public<Async>
}

impl ProvideProduct for CoinBaseClient {
    fn get_products(&self) -> Result<Vec<Product>, dyn Error> {
        client.get_products().await?
    }
}
#[tokio::main]
async fn main() {
    // Do some startup searching, spin up background thread to do work.
    let client: Public<ASync> = Public::new_with_keep_alive(MAIN_URL, false);
    let client = CoinBaseClient{client};
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

async fn get_currencies(client: &impl ProvideProduct, currency: &str) -> Result<Vec<ScannedProduct>, &str> {
    let products = client.get_products().await.unwrap();
    Ok(products
        .iter()
        .filter(|&p| p.quote_currency == currency)
        .map(|x| ScannedProduct {
            id: x.id.clone(),
            name: x.display_name.clone(),
            base_currency: x.base_currency.clone(),
            quote_currency: x.quote_currency.clone(),
        })
        .collect::<Vec<ScannedProduct>>())
}

async fn health_check(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("healthy".into()))
}
