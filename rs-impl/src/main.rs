#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use poem::{
    get, handler, listener::TcpListener, middleware::Tracing, web::Redirect, EndpointExt, Result,
    Route, Server,
};
use poem_openapi::OpenApiService;

mod api;
mod auth;
mod database;
mod models;
mod schema;

#[handler]
async fn redirect() -> Redirect {
    Redirect::moved_permanent("/swagger")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = database::get_connection_pool();

    let api_service =
        OpenApiService::new((api::ComicsApi, api::AuthApi), "Collection", "1.0").server("/api");
    let swagger = api_service.swagger_ui();
    let redoc = api_service.redoc();
    let server_key = auth::get_server_key();
    let app = Route::new()
        .at("/", get(redirect))
        .nest("/api", api_service)
        .nest("/swagger", swagger)
        .nest("/redoc", redoc)
        .with(Tracing)
        .data(server_key)
        .data(pool);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
