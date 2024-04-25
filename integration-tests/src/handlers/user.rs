use reqwest::Error;
use serde_json::Value;

use crate::dtos::user::{
    User,
    Token,
};

pub async fn register_new_user(user: &User) {

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

pub async fn login(user: User) -> Token {
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



pub async fn get_token_for(user: User) -> Result<Token, Error>{
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
