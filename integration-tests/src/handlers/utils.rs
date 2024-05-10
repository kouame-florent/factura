use factura::config::Config;
use factura::{config, handle_errors};
use serde_json::Value;
use uuid::Uuid;
use std::process::Command;
use std::io::{self, Write};
use sqlx::postgres::{PgPool, PgPoolOptions,};

use crate::dtos::user::{Token, PostUserRequest};
use crate::handlers::user::{
    login,
    register_new_user,
};

// pub async fn create_db(config: &Config) -> Result<(),handle_errors::Error>{

//     let s = Command::new("sqlx")
//     .arg("database")
//     .arg("create")
//     .arg("--database-url")
//     .arg(format!("postgres://{}:{}@{}:{}/{}",
//             config.db_user,
//             config.db_password,
//             config.db_host,
//             config.db_port,
//             config.db_name
//     ))
//     .output()
//     .expect("sqlx command failed to start");

//     // Exdcute DB commands to drop and create a new test database
//     io::stdout().write_all(&s.stderr).unwrap();

//     Ok(())



// }

// pub async fn drop_db(config: &Config)-> Result<(),handle_errors::Error>{

//     let db_url = format!("postgres://{}:{}@{}:{}/{}",
//         config.db_user,
//         config.db_password,
//         config.db_host,
//         config.db_port,
//         config.db_name
//     );

//     //println!("--- DB URL: {:?}",db_url.clone());

//     let s = Command::new("sqlx")
//         .arg("database")
//         .arg("drop")
//         .arg("--force")
//         .arg("--database-url")
//         .arg(db_url).arg("-y")
//         .output()
//         .expect("sqlx command failed to start");

//     io::stdout().write_all(&s.stderr).unwrap();

//     Ok(())


//  }


pub async fn  register_and_login(user: &PostUserRequest) -> Token {
    let client = reqwest::Client::new();
    let _ = client
        .post("http://localhost:3030/registrations")
        .json(&user)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await;
        
    let log_res = client
        .post("http://localhost:3030/login")
        .json(&user)
        .send()
        .await
        .unwrap();

    let token = log_res.json::<Token>()
        .await
        .unwrap();

    token
}


pub fn get_email() -> String {
    let login =  Uuid::new_v4().to_string();
    let server = "gmail.com".to_string();
    let email = format!("{login}@{server}");

    email

}