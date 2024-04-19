use std::process::Command;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use futures_util::future::FutureExt; 

use factura::{config, handle_errors, oneshot, setup_db_connection};


#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: String,
    email: String,
    password: String,
    roles: String,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Fournisseur{
    id: FournisseurId,
    code: String,
    sigle: String,
    designation: String,
    telephone: String,
    email: String,
    updated_by: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FournisseurId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FournisseurAnswer{
    id: FournisseurId,
    code: String,
    sigle: String,
    designation: String,
    telephone: String,
    email: String,
    updated_by: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Token(String);

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {

    dotenv::dotenv().ok();
    let config = config::Config::new().expect("Config can't be set");

    let s = Command::new("sqlx")
        .arg("database")
        .arg("drop")
        .arg("--database-url")
        .arg(format!("postgres://{}:{}@{}:{}/{}",
                config.db_user,
                config.db_password,
                config.db_host,
                config.db_port,
                config.db_name
        )).arg("-y")
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

     // set up a new store instance with a db connection pool
    let conn = setup_db_connection(&config).await?;

    // start the server and listen for a sender signal to shut it down
    let handler = oneshot(conn).await;

    let u = User {
        id: "aa-uu".to_string(),
        email: "test@email.com".to_string(),
        password: "password".to_string(),
        roles: "ADMIN".to_string(),
    };
        
    let token;

    print!("Running register_new_user...");
    let result = std::panic::AssertUnwindSafe(register_new_user(&u)).catch_unwind().await;
    match result {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running login...");
    match std::panic::AssertUnwindSafe(login(u)).catch_unwind().await {
        Ok(t) => {
            token = t;
            println!("✓");
        },
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }


    print!("Running post_fournisseur...");
    match std::panic::AssertUnwindSafe(post_fournisseur(token)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }


    let _ = handler.sender.send(1);

    Ok(())
}


async fn register_new_user(user: &User) {

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/registrations")
        .json(&user)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await;
        

    assert_eq!(res.unwrap(), "Account added".to_string());

   
}

async fn login(user: User) -> Token {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/login")
        .json(&user)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);

    res.json::<Token>()
        .await
        .unwrap()
}

async fn post_fournisseur(token: Token) {
    let f = Fournisseur {
        id: FournisseurId("aa-bb".to_string()),
        code: "f-01".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
        updated_by: "test@email.com".to_string()
    };

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0)
        .json(&f)
        .send()
        .await
        .unwrap()
        .json::<FournisseurAnswer>()
        .await
        .unwrap();

    assert_eq!(res.email, "sgb@gmail.com");
    assert_eq!(res.sigle, f.sigle);
}