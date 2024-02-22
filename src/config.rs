use std::env;

use clap::{Parser, ValueEnum};
use dotenv::dotenv;

use crate::errors::Error;

pub const GOOGLE_AI_KEY: &str = "GOOGLE_AI_KEY";
pub const PASETO_KEY: &str = "PASETO_KEY";
pub const PORT: &str = "PORT";
pub const POSTGRES_USER: &str = "POSTGRES_USER";
pub const POSTGRES_PASSWORD: &str = "POSTGRES_PASSWORD";
pub const POSTGRES_HOST: &str = "POSTGRES_HOST";
pub const POSTGRES_PORT: &str = "POSTGRES_PORT";
pub const POSTGRES_DB: &str = "POSTGRES_DB";

pub const DB_TYPE: &str = "DB_TYPE";

#[derive(ValueEnum, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
pub enum DatabaseType {
    Postgres,
    Memory,
}

/// Q&A web service API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Which errors we want to log (info, warn or error)
    #[clap(short, long, default_value = "warn")]
    pub log_level: String,
    /// Which PORT the server is listening to
    #[clap(short, long, default_value = "8080")]
    pub port: u16,
    /// Database user
    #[clap(long, default_value = "postgres")]
    pub db_user: String,
    /// Database user
    #[clap(long, default_value = "")]
    pub db_password: String,
    /// URL for the postgres database
    #[clap(long, default_value = "localhost")]
    pub db_host: String,
    /// PORT number for the database connection
    #[clap(long, default_value = "5432")]
    pub db_port: u16,
    /// Database name
    #[clap(long, default_value = "rush")]
    pub db_name: String,
    /// Database type
    #[clap(long, value_enum, default_value = "postgres")]
    pub db_type: DatabaseType,
}

impl Config {
    pub fn new() -> Result<Config, Error> {
        dotenv().ok();
        let config = Config::parse();

        if env::var(GOOGLE_AI_KEY).is_err() {
            panic!("Google_AI_KEY not set");
        }

        if env::var(PASETO_KEY).is_err() {
            panic!("PASETO_KEY not set");
        }

        let port = std::env::var(PORT)
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(config.port))
            .map_err(Error::ParseError)?;

        let db_user = env::var(POSTGRES_USER).unwrap_or(config.db_user.to_owned());
        let db_password = env::var(POSTGRES_PASSWORD).unwrap();
        let db_host = env::var(POSTGRES_HOST).unwrap_or(config.db_host.to_owned());
        let db_port = env::var(POSTGRES_PORT).unwrap_or(config.db_port.to_string());
        let db_name = env::var(POSTGRES_DB).unwrap_or(config.db_name.to_owned());
        let db_type = match env::var(DB_TYPE) {
            Ok(str) => match DatabaseType::from_str(&str, false) {
                Ok(t) => t,
                Err(_) => config.db_type.to_owned()
            },
            Err(_) => config.db_type.to_owned()
        };


        Ok(Config {
            log_level: config.log_level,
            port,
            db_user,
            db_password,
            db_host,
            db_port: db_port.parse::<u16>().map_err(Error::ParseError)?,
            db_name,
            db_type,
        })
    }
}
