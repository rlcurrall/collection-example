use poem_openapi::{payload::Json, OpenApi};

use crate::api::ApiTags;
use crate::auth::{JwtAuthorization, User};

pub struct AuthApi;

#[OpenApi]
impl AuthApi {
    /// Get the current user
    ///
    /// This uses the bearer token to identify the user and returns the claims.
    #[oai(path = "/me", method = "get", tag = "ApiTags::Authentication")]
    async fn get_me(&self, auth: JwtAuthorization) -> Json<User> {
        Json(auth.0)
    }
}
