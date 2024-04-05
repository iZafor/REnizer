use sqlx::{mysql::MySqlConnectOptions, *};
use once_cell::sync::Lazy;

pub static mut G_STATE: Lazy<GState> = Lazy::new(|| GState::default()); 

pub struct GState {
    pub db_conn: Option<Pool<MySql>> 
}

impl GState {
    pub async fn get_db_conn(db_config: DBConfig) -> Result<Pool<MySql>, sqlx::Error> {
        MySqlPool::connect_with(
            MySqlConnectOptions::new()
            .host(db_config.host)
            .port(db_config.port)   
            .username(db_config.username)
            .password(db_config.password)
            .database(db_config.database)
        ).await
    }
}

impl Default for GState {
    fn default() -> Self {
        GState { db_conn: None }
    }
}

pub struct DBConfig {
    pub host: &'static str,
    pub port: u16,
    pub username: &'static str,
    pub password: &'static str,
    pub database: &'static str 
}

impl DBConfig {
    pub fn new() -> Self {
        DBConfig {
            host: env!("REnizer_DB_HOST"),
            port: env!("REnizer_DB_PORT").parse().unwrap(),
            username: env!("REnizer_DB_USER_NAME"),
            password: env!("REnizer_DB_USER_PASSWORD"),
            database: "REnizer"
        }
    }
}