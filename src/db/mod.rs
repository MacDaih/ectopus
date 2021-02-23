use mongodb::{
    error::Error,
    options::{ClientOptions},
    Client
};

use bson::{doc};

pub mod model;

pub mod service;
use service::{
    ReaderService
};

pub async fn db() -> mongodb::error::Result<ReaderService> {
    let uri = "mongodb://localhost:27017/";
    let options = ClientOptions::parse(uri).await.unwrap();
    let client = Client::with_options(options).unwrap();
    let db_name = "ectopus";
    ping_db(&db_name, &client)
        .await
        .expect("could not connect to database");
    let db = client.database(&db_name);
    let collection_name = "reports";
    let collection = db.collection(&collection_name);
    Ok(ReaderService::new(collection.clone()))
}

async fn ping_db(db_name: &str, cli: &Client) -> Result<(), Error> {
    let p = cli
        .database(db_name)
        .run_command(doc! {"ping": 1}, None)
        .await;
    match p {
        Ok(_p) => Ok(()),
        Err(e) => panic!(e),
    }
}
