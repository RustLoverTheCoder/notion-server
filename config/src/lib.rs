pub mod contants;

use std::env;

use anyhow::Ok;
use async_redis_session::RedisSessionStore;
use contants::{DB, JWT_SECRET, REDIS_SESSION_STORE};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

pub async fn init() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect(".env must have DATABASE_URL");

    let redis_url = env::var("REDIS_URL").expect(".env must have REDIS_URL");

    let jwt_secret = env::var("JWT_SECRET").expect(".env must have JWT_SECRET");

    // let jwt_secret_bytes = jwt_secret.as_bytes();

    let db: DatabaseConnection = Database::connect(&database_url)
        .await
        .expect("Database connection failed");


    let redis_session_store = RedisSessionStore::new(redis_url).expect("Redis connection failed");

    JWT_SECRET.set(jwt_secret).unwrap();

    REDIS_SESSION_STORE.set(redis_session_store).unwrap();

    DB.set(db).unwrap();

    let db = DB.get().unwrap();

    Migrator::up(db, None).await.unwrap();

    Ok(())
}