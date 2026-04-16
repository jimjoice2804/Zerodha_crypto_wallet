use actix_web::{HttpResponse, Responder, get};
use serde::Serialize;

#[derive(Serialize)]
struct NonceResponse {
    nonce: String,
}

#[get("/health")]
pub async fn healthz() -> impl Responder {
    let response_data = NonceResponse {
        nonce: "random_string_123".to_string(),
    };

    HttpResponse::Ok().json(response_data)
}
