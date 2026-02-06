use std::env;
pub struct Config{
    pub mongodb_uri:String,
    pub db_name:String,
    pub server_port:String,
    pub jwt_secret:String
}

impl Config {
    pub fn init()->Self{
        Self{
            mongodb_uri: env::var("MONGODB_URI").expect("MONGODB_URI ayarlanmamış"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET ayarlanmamış"),
            db_name:env::var("DATABASE_NAME").expect("DATABASE_NAME ayarlanmamış"),
            server_port:env::var("PORT").unwrap_or_else(|_|"3000".to_string())
        }
    }
}