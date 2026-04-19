use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;
extern crate bcrypt;
use bcrypt::verify;
use jsonwebtoken::{EncodingKey, Header, encode};
use std::env;

#[derive(Validate, Deserialize)]
struct Info {
    #[validate(length(min = 3, max = 20))]
    email: String,
    #[validate(length(min = 3, max = 20))]
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[post("/login")]
async fn login_account(info: web::Json<Info>, pool: web::Data<PgPool>) -> impl Responder {
    match info.validate() {
        Ok(_) => {}
        Err(error) => return HttpResponse::BadRequest().json(error),
    };

    let Info { email, password } = info.into_inner();

    let user_exist = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE email = $1",
        email
    )
    .fetch_optional(pool.get_ref())
    .await;

    let user_option = match user_exist {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json("Something went wrong while fetching data from source");
        }
    };

    let user_option = match user_option {
        Some(row) => row,
        None => return HttpResponse::Unauthorized().json("Invalid email or password"),
    };

    let valid = verify(password, &user_option.password_hash);

    let valid_result = match valid {
        Ok(is_match) => is_match,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json("Something went wrong while verifying!!");
        }
    };

    if !valid_result {
        return HttpResponse::Unauthorized().json("Invalid email or password");
    }
    let header = Header::default();

    let expiration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + (24 * 60 * 60);
    let claims = Claims {
        sub: email,
        company: "Zerodha".to_string(),
        exp: expiration,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token_encode = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    );

    let token = match token_encode {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to create token"),
    };

    return HttpResponse::Created().json(serde_json::json!({
        "token": token
    }));
}
