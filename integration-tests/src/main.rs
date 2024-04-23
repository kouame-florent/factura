use std::process::Command;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
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
struct PostFournisseurRequest{
    pub code: String,
    pub sigle: String,
    pub designation: String,
    pub telephone: String,
    pub email: String,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PostFournisseurAnswer{
    pub id: FournisseurId,
    pub code: String,
    pub sigle: String,
    pub designation: String,
    pub telephone: String,
    pub email: String,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub updated_by: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct PutFournisseurAnswer{
    pub id: FournisseurId,
    pub code: String,
    pub sigle: String,
    pub designation: String,
    pub telephone: String,
    pub email: String,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub updated_by: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PutFournisseurRequest{
    pub code: String,
    pub sigle: String,
    pub designation: String,
    pub telephone: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FournisseurId(pub String);


#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetFournisseurAnswer{
    pub id: String,
    pub code: String,
    pub sigle: String,
    pub designation: String,
    pub telephone: String,
    pub email: String,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub updated_by: String,
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
    match std::panic::AssertUnwindSafe(post_fournisseur(token.clone())).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running get_fournisseurs...");
    match std::panic::AssertUnwindSafe(get_fournisseurs(token.clone())).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running get_fournisseur (get fournisseur by id)...");
    match std::panic::AssertUnwindSafe(get_fournisseur(token.clone())).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running get_fournisseur (get fournisseur by id with a wrong id)...");
    match std::panic::AssertUnwindSafe(get_fournisseur_with_wrong_id(token.clone())).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running put_fournisseur...");
    match std::panic::AssertUnwindSafe(put_fournisseur(token.clone())).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running delete_fournisseur...");
    match std::panic::AssertUnwindSafe(delete_fournisseur(token.clone())).catch_unwind().await {
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
    let f = PostFournisseurRequest {
        code: "f-01".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
    
    };

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0)
        .json(&f)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();

    assert_eq!(res.email, "sgb@gmail.com");
    assert_eq!(res.sigle, f.sigle);
}


async fn get_fournisseurs(token: Token){

    let f1 = PostFournisseurRequest {

        code: "f-01".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
    
    };

    let f2 = PostFournisseurRequest{
 
        code: "f-02".to_string(),
        sigle: "GHI".to_string(),
        designation: "societe du sud".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "ghi@gmail.com".to_string(),
       
    };

    let f3 = PostFournisseurRequest {

        code: "f-03".to_string(),
        sigle: "GTI".to_string(),
        designation: "societe du sud".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "gti@gmail.com".to_string(),
    
    };

    let client = reqwest::Client::new();

    client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f1)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();

    client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f2)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();


    client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f3)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();


    let res = client
        .get("http://localhost:3030/fournisseurs?limit=2&offset=0")
        .header("Authorization", token.0.clone())
        .send()
        .await
        .unwrap()
        .json::<Vec<GetFournisseurAnswer>>()
        .await
        .unwrap();

    assert_eq!(res.len(),2);

    let res = client
        .get("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .send()
        .await
        .unwrap()
        .json::<Vec<GetFournisseurAnswer>>()
        .await
        .unwrap();

    assert_eq!(res.len(),4); //4 because of the previous add_fournisseur test
}

async fn get_fournisseur(token: Token){

    let f1 = PostFournisseurRequest {
        code: "f-05".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
       
    };

    let client = reqwest::Client::new();

    let post_res = client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f1)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();

    let raw_url = "http://localhost:3030/fournisseurs/".to_owned();
    let id = post_res.id.0.clone();
    let get_url = format!("{raw_url}{id}");

    let get_res = client
        .get(get_url)
        .header("Authorization", token.0.clone())
        .send()
        .await
        .unwrap()
        .json::<GetFournisseurAnswer>()
        .await
        .unwrap();

    assert_eq!(get_res.id,post_res.id.0);


}

async fn get_fournisseur_with_wrong_id(token: Token){

    let f1 = PostFournisseurRequest {
        code: "f-05".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
       
    };

    let client = reqwest::Client::new();

    client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f1)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();

    let raw_url = "http://localhost:3030/fournisseurs/".to_owned();
    let id = "aaa-bbb";
    let get_url = format!("{raw_url}{id}");

    let get_res = client
        .get(get_url)
        .header("Authorization", token.0.clone())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

        
    assert_eq!(get_res,"Entity not found");


}


async fn put_fournisseur(token: Token){

    let f1 = PostFournisseurRequest {

        code: "f-01".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
   
    };

    let client = reqwest::Client::new();
    let post_res = client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f1)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();

    
    let f2 = PutFournisseurRequest {
            code: "f-02".to_string(),
            sigle: "ASB".to_string(),
            designation: "Association des societés".to_string(),
            telephone: "07-08-08-08-08".to_string(),
            email: "asb@gmail.com".to_string(),
    
    };

    let raw_url = "http://localhost:3030/fournisseurs/".to_owned();
    let id = post_res.id.0;
    let put_url = format!("{raw_url}{id}");

    let put_res = client
        .put(put_url)
        .header("Authorization", token.0)
        .json(&f2)
        .send()
        .await
        .unwrap()
        .json::<PutFournisseurAnswer>()
        .await
        .unwrap();

    assert_eq!(put_res.code,"f-02");
    assert_eq!(put_res.sigle,"ASB");
    assert_eq!(put_res.designation,"Association des societés");
    assert_eq!(put_res.telephone,"07-08-08-08-08");
    assert_eq!(put_res.email,"asb@gmail.com");



}


async fn delete_fournisseur(token: Token){
    let f1 = PostFournisseurRequest {

        code: "f-09".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
   
    };

    let client = reqwest::Client::new();
    let post_res = client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f1)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();

    let raw_url = "http://localhost:3030/fournisseurs/".to_owned();
    let id = post_res.id.0;
    let delete_url = format!("{raw_url}{id}");

    let del_resp = client
        .delete(delete_url)
        .header("Authorization", token.0.clone())
        .send()
        .await
        .unwrap()
        .json::<bool>()
        .await
        .unwrap();

    assert_eq!(del_resp,true);

}