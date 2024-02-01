pub mod router;
pub mod controller;
pub mod model;
pub mod utils;


use salvo::prelude::*;
use crate::model::mongo::mongodb_init;
use crate::router::router::{get_router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    mongodb_init().await;

    let router = get_router();

    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}

