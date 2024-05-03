use factura::config::{self, Config};
use reqwest::Error;
use serde_json::Value;


use crate::{dtos::user::{PostUserRequest, Token}, handlers::utils::{create_db, get_email}};


pub async fn register_new_user() {

    //reset_db(config).await.unwrap();

    let user = PostUserRequest {
        email: get_email(),
        password: "password".to_string(),
        roles: "ADMIN,CE,DAFP".to_string(),
    };

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

   // drop_db(config).await.unwrap();
}

pub async fn login() {
   
     let user = PostUserRequest {
        
        email: get_email(),
        password: "password".to_string(),
        roles: "ADMIN,CE,DAFP".to_string(),
    };

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
