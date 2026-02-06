use jsonwebtoken::{encode,decode,Header,Validation,EncodingKey,DecodingKey};
use serde::{Deserialize,Serialize};
use std::time::{SystemTime,UNIX_EPOCH};

#[derive(Debug,Serialize,Deserialize)]
pub struct Cliams{
    pub role:String,
    pub sub:String,
    pub exp:usize,
    pub iat:usize
}

pub fn create_jwt(username:&str,role:&str,secret:&str)->Result<String,jsonwebtoken::errors::Error>{
    let now =SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Zaman geri gidemez")
        .as_secs() as usize;
    let claims=Cliams{
        sub:username.to_owned(),
        role:role.to_owned(),
        exp:now+(24*3600),
        iat:now,
    };
    encode(&Header::default(),&claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn verify_jwt(token:&str,secret:&str)->Result<Cliams,jsonwebtoken::errors::Error>{
    decode::<Cliams>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default()
    ).map(|data|data.claims)
}