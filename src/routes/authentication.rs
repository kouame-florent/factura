
use std::env;
use std::future;

use chrono::Utc; 
use tracing::instrument;
use argon2::{self, Config};
use rand::Rng;
use warp::Filter;

use crate::store::authentication::AuthStore;
use crate::types::account::Account;
use crate::types::account::AccountId;
use crate::types::account::NewAccount;
use crate::types::account::Session;



#[instrument]
pub async fn register( 
    store: AuthStore,
    account: NewAccount,
) -> Result<impl warp::Reply, warp::Rejection>{

    let hashed_password = hash_password(account.password.as_bytes());

    let account = NewAccount {
        email: account.email,
        password: hashed_password, 
        roles: account.roles,
    };

    match store.add_account(account).await {
        Ok(_) => {
            Ok(warp::reply::json(&"Account added".to_string()))
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

#[instrument]
pub async fn login(
    store: AuthStore,
    login: Account,
) -> Result<impl warp::Reply, warp::Rejection> {
   
    match store.get_account(login.email).await {
        Ok(account) => match verify_password(
            &account.password,
            login.password.as_bytes(),
        ) {
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(
                        account.id.expect("id not found"),
                        //login.roles.expect("roles not found"),
                    )))
                } else {
                    Err(warp::reject::custom(
                        handle_errors::Error::WrongPassword,
                    ))
                }
            }
            Err(e) => Err(warp::reject::custom(
                handle_errors::Error::ArgonLibraryError(e),
            )),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}


pub fn verify_token(
    token: String,
) -> Result<Session, handle_errors::Error> {
    let key = env::var("PASETO_KEY").unwrap();
    let token = paseto::tokens::validate_local_token(
        &token,
        None,
        key.as_bytes(),
        &paseto::tokens::TimeBackend::Chrono,
    )
    .map_err(|_| handle_errors::Error::CannotDecryptToken)?;
    serde_json::from_value::<Session>(token)
        .map_err(|_| handle_errors::Error::CannotDecryptToken)
}

fn verify_password(
    hash: &str,
    password: &[u8]
) -> Result<bool, argon2::Error> {
     argon2::verify_encoded(hash, password)
}

fn issue_token(
    account_id: AccountId,
    //roles: String,
) -> String {
    
        let key = env::var("PASETO_KEY").unwrap();
        
        let current_date_time = Utc::now();
        let dt = current_date_time + chrono::Duration::days(1);

        paseto::tokens::PasetoBuilder::new()
            .set_encryption_key(&Vec::from(key.as_bytes()))
            .set_expiration(&dt)
            .set_not_before(&Utc::now())
            .set_claim("account_id", serde_json::json!(account_id))
            //.set_claim("roles", serde_json::json!(roles))
            .build()
            .expect("Failed to construct paseto token w/ builder!")
}

#[instrument]
pub fn auth() -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization").and_then(|token: String| {
        let token= match verify_token(token) {
            Ok(t) => t,
            Err(_) => {
                return future::ready(Err(warp::reject::custom(
                    handle_errors::Error::Unauthorized,
                )))
            }
        };

        future::ready(Ok(token))
    })
}