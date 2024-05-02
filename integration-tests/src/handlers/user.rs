use factura::config::{self, Config};
use reqwest::Error;
use serde_json::Value;

use crate::{dtos::user::{
    Token, PostUserRequest
}, handlers::test_init::init_db};

pub async fn register_new_user(user: &PostUserRequest, config: &Config) {

    init_db(config).unwrap();
    // let user = User {
    //     id: "aa-xu".to_string(),
    //     email: "test@email.com".to_string(),
    //     password: "password".to_string(),
    //     roles: "ADMIN,CE,DAFP".to_string(),
    // };

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

pub async fn login(user: &PostUserRequest, config: &Config) -> Token {

    init_db(config).unwrap();
    // let user = User {
    //     id: "aa-xu".to_string(),
    //     email: "test@email.com".to_string(),
    //     password: "password".to_string(),
    //     roles: "ADMIN,CE,DAFP".to_string(),
    // };

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

    assert_eq!(log_res.status(), 200);

    log_res.json::<Token>()
        .await
        .unwrap()
}



pub async fn get_token_for(user: PostUserRequest) -> Result<Token, Error>{
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

    log_res.json::<Token>()
        .await
        
        

}
