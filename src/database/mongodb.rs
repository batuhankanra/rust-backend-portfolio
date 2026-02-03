use mongodb::{Client,Database};
use crate::config::env::Config;

pub struct MongoRepo{
    #[allow(dead_code)]
    pub db:Database
}

impl MongoRepo {
    pub async fn init()->Self {
        let config=Config::init();
        let client=Client::with_uri_str(&config.mongodb_uri).await.expect("MongoDb bağlantısı başarısız");
        let db=client.database(&config.db_name);
        println!("\x1b[1;32m✔ MongoDB bağlantısı başarılı: {}\x1b[0m", config.db_name);
        Self {db}

    }
}