use diesel::{
    r2d2::{Builder, ConnectionManager, Pool},
    PgConnection,
};
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let max_size = env::var("CONNECTION_POOL_MAX_SIZE")
        .map(|s| str::parse::<u32>(&s).unwrap_or(64))
        .unwrap_or(64);
    let min_idle = env::var("CONNECTION_POOL_MIN_IDLE")
        .map(|s| str::parse::<u32>(&s).unwrap_or(32))
        .unwrap_or(32);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager: ConnectionManager<PgConnection> = ConnectionManager::new(database_url);
    Builder::new()
        .max_size(max_size)
        .min_idle(Some(min_idle))
        .build(manager)
        .expect("Connection pool initialization failed")
}
