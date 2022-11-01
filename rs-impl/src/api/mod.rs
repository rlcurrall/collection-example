mod auth;
mod comic;

pub use auth::AuthApi;
pub use comic::ComicsApi;
use poem_openapi::{Enum, Tags};

#[derive(Tags)]
enum ApiTags {
    Comics,
    Authentication,
}

#[derive(Enum)]
enum OrderDirection {
    Asc,
    Desc,
}
