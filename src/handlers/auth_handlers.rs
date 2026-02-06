use std::sync::Arc;

use axum::{http::StatusCode,Json,extract::State};
use bcrypt::{DEFAULT_COST, hash, verify};
use mongodb::bson::doc;
use serde::{Deserialize,Serialize};
use crate::{auth::jwt::create_jwt, config::env::Config, database::mongodb::MongoRepo, error::{AppError, MyJson}};
use serde_json::{Value, json};
use crate::models::user_models::User;


#[derive(Deserialize,Serialize)]
pub struct LoginRequest{
    pub email:String,
    pub password:String
}
pub async fn login_handlers(State(db):State<Arc<MongoRepo>>,MyJson(payload):MyJson<LoginRequest>)->Result<(StatusCode,Json<Value>),AppError> {
    let collection=db.db.collection::<User>("users");
    let config=Config::init();

    let user=collection.find_one(doc! {"email":&payload.email}).await?.ok_or(AppError::Unauthorized)?;

    let is_valid=verify(&payload.password, &user.password_hash).map_err(|_| AppError::HashError)?;
    if !is_valid {
        return Err(AppError::Unauthorized);
    }
    let token=create_jwt(&user.username,&user.role,&config.jwt_secret).map_err(|_| AppError::HashError)?;
    Ok((
        StatusCode::OK, 
        Json(json!({
            "status": "success",
            "token": token,
            "user": {
                "username": user.username,
                "email": user.email,
                "role": user.role
            }
        }))
    ))


}



#[derive(Deserialize,Serialize)]
pub struct RegisterRequest{
    pub email:String,
    pub username:String,
    pub password:String
}

pub async fn register(State(db):State<Arc<MongoRepo>>,MyJson(payload):MyJson<RegisterRequest>)->Result<(StatusCode, Json<Value>), AppError>{
    let collection=db.db.collection::<User>("users");
    let existing_user=collection.find_one(doc! {"email":&payload.email}).await?;
    if existing_user.is_some(){
        return Err(AppError::UserAlreadyExists);
    }
    let hashed_password=hash(&payload.password, DEFAULT_COST).map_err(|_| AppError::HashError)?;
    let new_user=User{
        id:None,
        email:payload.email,
        username:payload.username,
        password_hash:hashed_password,
        role:"user".to_string()
    };
    collection.insert_one(new_user).await.map_err(AppError::MongoError)?;
    Ok((
        StatusCode::CREATED,
        Json(json!({"status":"success","msg":"Kullanıcı başarıyla kaydedildi"}))
    ))

} 