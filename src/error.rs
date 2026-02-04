use axum::{
    extract::{FromRequest, Request},
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, // Axum Json wrapper'ı
};
use serde_json::json;
use thiserror::Error;

// 1. Hata Tiplerini Tanımlıyoruz
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Veritabanı hatası: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("Geçersiz ID: {0}")]
    InvalidId(String),

    #[error("Kayıt bulunamadı")]
    NotFound,
    #[error("{0}")]
    JsonRejection(String),
}

// 2. Hata -> HTTP Yanıtı Dönüşümü
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::MongoError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::InvalidId(id) => (StatusCode::BAD_REQUEST, format!("Geçersiz ID: {}", id)),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Kayıt bulunamadı".to_string()),
            // Gelen string mesajı olduğu gibi alıyoruz
            AppError::JsonRejection(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        // Cevabın bir JSON objesi olmasını garanti ediyoruz
        let body = Json(json!({
            "status": "error",
            "message": error_message
        }));

        (status, body).into_response()
    }
}
pub struct MyJson<T>(pub T);

impl<S, T> FromRequest<S> for MyJson<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            
            // HATA BURADA:
            // rejection.body_text() yerine doğrudan hatayı string'e çeviriyoruz.
            // Bu sayede "JSON görünümlü string" karmaşası oluşmaz.
            Err(rejection) => {
                let error_text = rejection.body_text();
                Err(AppError::JsonRejection(error_text))
            }
        }
    }
}