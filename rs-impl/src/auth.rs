use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use poem::Request;
use poem_openapi::{auth::Bearer, Object, SecurityScheme};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

pub type ServerKey = Hmac<Sha256>;

#[derive(Debug, Serialize, Deserialize, Object)]
pub struct User {
    pub username: String,
}

#[derive(SecurityScheme)]
#[oai(
    type = "bearer",
    key_name = "Authorization",
    in = "header",
    checker = "verify_token"
)]
pub struct JwtAuthorization(pub User);

pub async fn verify_token(req: &Request, bearer: Bearer) -> Option<User> {
    let server_key = req.data::<ServerKey>().unwrap();
    VerifyWithKey::<User>::verify_with_key(bearer.token.as_str(), server_key).ok()
}

pub fn get_server_key() -> ServerKey {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET variable not set");
    ServerKey::new_from_slice(jwt_secret.as_bytes()).expect("valid server key")
}
