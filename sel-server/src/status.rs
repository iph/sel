use async_trait::async_trait;
use coinbase_pro_rs::structs::public::Product;
use coinbase_pro_rs::{ASync, CBError, Public, MAIN_URL};
use hyper::{service::make_service_fn, service::service_fn, Body, Request, Response, Server};
use std::convert::Infallible;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::time::Instant;

#[derive(PartialEq, Clone, Debug)]
pub struct ScannedProduct {
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
#[derive(PartialEq, Clone, Debug)]
pub enum ExecutionStatus {
    // Waiting is within the time of execution,
    Waiting,
    PendingFill,
    Executed,
    PendingExit,
    Exited,
    Expired,
    Error,
}

#[async_trait]
pub trait ProvideProduct {
    async fn get_products(&self) -> Result<Vec<Product>, CBError>;
}

impl Default for ExecutionStatus {
    fn default() -> Self {
        ExecutionStatus::Waiting
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct StoredBet {
    id: String,
    end_date: Instant,
    enter_amount: f64,
    stop_loss: f64,
    stop_gain: f64,
    status: ExecutionStatus,
}

pub struct CoinBaseClient {
    pub client: Public<ASync>,
}

#[async_trait]
impl ProvideProduct for CoinBaseClient {
    async fn get_products(&self) -> Result<Vec<Product>, CBError> {
        self.client.get_products().await
    }
}

pub async fn get_currencies(
    client: &impl ProvideProduct,
    currency: &str,
) -> Result<Vec<ScannedProduct>, CBError> {
    let products = client.get_products().await?;
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