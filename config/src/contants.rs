use async_redis_session::RedisSessionStore;
use once_cell::sync::{Lazy, OnceCell};
use regex::Regex;
use sea_orm::DbConn;

pub static DB: OnceCell<DbConn> = OnceCell::new();

pub static JWT_SECRET: OnceCell<String> = OnceCell::new();

pub static REDIS_SESSION_STORE: OnceCell<RedisSessionStore> = OnceCell::new();

pub static PHONE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^1[3456789]\d{9}$").unwrap());