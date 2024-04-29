use std::env;
use clap::Parser;

#[derive(Parser, Debug, PartialEq)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Which errors we want to log (info, warn or error)
    #[clap(short, long, default_value = "warn")]
    pub log_level: String,
    /// Which PORT the server is listening to
    #[clap(short, long, default_value = "8080")]
    pub app_port: u16,
    /// Database user
    #[clap(long, default_value = "username")]
    pub db_user: String,
    /// Database user
    #[clap(long, default_value = "password")]
    pub db_password: String,
    /// URL for the postgres database
    #[clap(long, default_value = "localhost")]
    pub db_host: String,
    /// PORT number for the database connection
    #[clap(long, default_value = "5432")]
    pub db_port: u16,
    /// Database name
    #[clap(long, default_value = "factura")]
    pub db_name: String,
}


impl Config {
    pub fn new() -> Result<Config, handle_errors::Error> {

        let config = Config::parse();
        
        let log_level = env::var("RUST_LOG").unwrap_or_else(|_| config.log_level.to_owned());
        let db_user = env::var("DATABASE_USER").unwrap_or_else(|_| config.db_user.to_owned());
        let db_password = env::var("DATABASE_PASSWORD").unwrap();
        let db_host = env::var("DATABASE_HOST").unwrap_or_else(|_| config.db_host.to_owned());
        let db_port = env::var("DATABASE_PORT").unwrap_or_else(|_| config.db_port.to_string());
        let db_name = env::var("DATABASE_NAME").unwrap_or_else(|_| config.db_name.to_owned());
        let app_port = env::var("APPLICATION_PORT").unwrap_or_else(|_| config.app_port.to_string());



        Ok(Config{
            log_level: log_level,
            app_port: app_port
                .parse::<u16>()
                .map_err(handle_errors::Error::ParseError)?,
            db_user,
            db_password,
            db_host,
            db_port: db_port
                .parse::<u16>()
                .map_err(handle_errors::Error::ParseError)?,
            db_name,
        })

    }

}