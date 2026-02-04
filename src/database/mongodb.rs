use std::time::Duration;

use mongodb::{Client,Database, bson::doc, options::ClientOptions};
use crate::config::env::Config;

pub struct MongoRepo{
    #[allow(dead_code)]
    pub db:Database
}

impl MongoRepo {
    pub async fn init()->Self {
        let config=Config::init();
        let mut client_options=ClientOptions::parse(&config.mongodb_uri)
        .await
        .expect("MOngoDb uri parse hatası");
    client_options.server_selection_timeout=Some(Duration::from_secs(6));
    let client =Client::with_options(client_options).expect("İstemci oluşturulamadı");
    client.database("admin").run_command(doc! {"ping":1}).await.expect("\n❌ MongoDB sunucusuna ulaşılamadı! (Zaman aşımı: 2s)\n");
    let db=client.database(&config.db_name);

println!("\x1b[1;32m✔ MongoDB bağlantısı başarılı: {}\x1b[0m", config.db_name);
        
        Self { db }

    }
}