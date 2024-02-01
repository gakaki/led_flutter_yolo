use once_cell::sync::Lazy;
use rbatis::RBatis;
use redis::Client;
use salvo::conn::TcpListener;
use salvo::{Listener, Server};
use salvo::http::request::set_secure_max_size;
use tracing;
use mongodb::{bson::{Document, doc}, Client as MongoClient, Collection, Database};
mod controller;
mod entity;
mod mapper;
mod model;
mod router;
mod service;
mod utils;

mod websocket;

pub static GLOBAL_DB: Lazy<RBatis> = Lazy::new(|| RBatis::new());

pub static GLOBAL_REDIS: Lazy<Client> =
    Lazy::new(|| Client::open("redis://127.0.0.1/").expect("连接redis失败"));

//greptimedb
// pub static GLOBAL_GREPTIMEDB: Lazy<Client> =
//     Lazy::new(|| Client::open("redis://127.0.0.1/").expect("连接redis失败"));

//mongodb
pub static GLOBAL_MONGODB: Lazy<Database> =
    Lazy::new(||async {
        let uri = "mongodb://root:root@127.0.0.1:27017/?authSource=admin";
        let client = MongoClient::with_uri_str(uri).await?;
        let database = client.database("experiment");
        let my_coll: Collection<Document> = database.collection("experiments");
        let my_movie = my_coll.find_one(doc! { "title": "The Perils of Pauline" }, None).await?;
        // Print the document
        println!("Found a movie:\n{:#?}", my_movie);
        database
    });

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // 连接数据库
    utils::mysql::init_db().await;
    tracing::info!("数据库连接成功");

    // 连接redis
    GLOBAL_REDIS.get_connection().expect("连接redis失败");
    tracing::info!("redis连接成功");

    // 连接mongodb
    GLOBAL_MONGODB.collection("experiments").expect("连接mongodb失败");
    tracing::info!("mongodb连接成功");

    // // 连接greptimedb
    // GLOBAL_REDIS.get_connection().expect("连接redis失败");
    // tracing::info!("redis连接成功");

    let service = router::init_service();

    // 设置服务器最大接收数据
    set_secure_max_size(64 * 1024 * 100);

    Server::new(TcpListener::new("0.0.0.0:8090").bind().await)
        .serve(service)
        .await;
}
