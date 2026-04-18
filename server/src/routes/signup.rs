use actix_web::{HttpResponse, Responder, post, web};
use bcrypt;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};
use validator::Validate;

#[derive(Validate, Deserialize)]
struct Info {
    #[validate(length(min = 3, max = 20))]
    user_name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
    #[validate(length(min = 3, max = 20))]
    first_name: String,
    #[validate(length(min = 3, max = 20))]
    last_name: String,
}
#[derive(Serialize)]
struct UserResponse {
    id: u32,
    user_name: String,
    first_name: String,
    last_name: String,
    email: String,
    token: String,
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[post("/create")]
async fn create_account(info: web::Json<Info>, pool: web::Data<PgPool>) -> impl Responder {
    match info.validate() {
        Ok(_) => {}
        Err(error) => return HttpResponse::BadRequest().json(error),
    }
    let Info {
        email,
        password,
        user_name,
        first_name,
        last_name,
    } = info.into_inner();

    let hashed_password = bcrypt::hash(&password, 4).expect("Failed to hash the password");

    let query_result  =   sqlx::query!(
        "INSERT INTO users (user_name, email, password_hash, first_name, last_name) VALUES ($1,$2,$3,$4,$5) RETURNING id",
        user_name,
        email,
        hashed_password,
        first_name,
        last_name
    ).fetch_one(pool.get_ref()).await;

    let row = match query_result {
        Ok(data) => data,
        Err(_) => return HttpResponse::Conflict().json("Email or username Already exists"),
    };

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize
        + (24 * 60 * 60);

    let claims = Claims {
        sub: row.id.to_string(),
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT SECRET must be set");
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();

    let response = UserResponse {
        id: row.id as u32,
        email,
        user_name,
        first_name,
        last_name,
        token,
    };

    return HttpResponse::Created().json(response);
}
