use factura::config::Config;
use factura::{config, handle_errors};
use std::process::Command;
use std::io::{self, Write};

use crate::dtos::user::{Token, PostUserRequest};
use crate::handlers::user::{
    login,
    register_new_user,
};


pub fn init_db(config: &Config) -> Result<(),handle_errors::Error>{

    // dotenv::dotenv().ok();
    // let config = config::Config::new().expect("Config can't be set");

    let db_url = format!("postgres://{}:{}@{}:{}/{}",
        config.db_user,
        config.db_password,
        config.db_host,
        config.db_port,
        config.db_name
    );

    println!("--- DB URL: {:?}",db_url.clone());

    let s = Command::new("sqlx")
        .arg("database")
        .arg("drop")
        .arg("--database-url")
        .arg(db_url).arg("-y")
        .output()
        .expect("sqlx command failed to start");

    io::stdout().write_all(&s.stderr).unwrap();

    let s = Command::new("sqlx")
        .arg("database")
        .arg("create")
        .arg("--database-url")
        .arg(format!("postgres://{}:{}@{}:{}/{}",
                config.db_user,
                config.db_password,
                config.db_host,
                config.db_port,
                config.db_name
        ))
        .output()
        .expect("sqlx command failed to start");

    // Exdcute DB commands to drop and create a new test database
    io::stdout().write_all(&s.stderr).unwrap();

    println!("--- finishing db init");

    Ok(())
}


pub async fn init_user(user: &PostUserRequest, config: &Config) -> Result<Token,handle_errors::Error>{
    register_new_user(user, config).await;
    let token = login(user,config).await;
    Ok(token)
}