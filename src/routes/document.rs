
use std::collections::HashMap;

use tracing::{event, instrument, Level};


use crate::{
    store::document::DocumentStore, types::{
         document::{ NewDocument, UpdatedDocument}, pagination::{extract_pagination, 
        Pagination} 
    }
};

use crate::types::account::{Session,Roles};
use crate::store::authentication::AuthStore;



#[instrument]
pub async fn add_document(
    session: Session,
    store: DocumentStore,
    auth_store: AuthStore,
    document: NewDocument,
) -> Result<impl warp::Reply, warp::Rejection>{
    
    let account_id = session.account_id;
    let email = auth_store.get_email(account_id.0.clone()).await?;
    let admin: Roles = Roles::ADMIN;

    if auth_store.has_authorization(account_id.0, admin.as_str().to_string()).await?{
        match store.add_document(document, email.to_string()).await {
            Ok(f) => {
                Ok(warp::reply::json(&f))
            }
            Err(e) => Err(warp::reject::custom(e)),
        }

    }else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
    
}



#[instrument]
pub async fn update_document(
    id: String,
    session: Session,
    store: DocumentStore,
    auth_store: AuthStore,
    updated_document: UpdatedDocument,
) -> Result<impl warp::Reply, warp::Rejection>{

    let account_id = session.account_id;
    let email = auth_store.get_email(account_id.0.clone()).await?;
    let admin: Roles = Roles::ADMIN;
    if auth_store.has_authorization(account_id.0, admin.as_str().to_string()).await?{

        match store.update_document(updated_document, id, email).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e))
        }
    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }

  
}

#[instrument]
pub async fn get_documents(
    session: Session,
    params: HashMap<String, String>,
    store: DocumentStore,
    auth_store: AuthStore,
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying document");

    let account_id = session.account_id;
    let role: Roles = Roles::CE;
    if auth_store.has_authorization(account_id.0, role.as_str().to_string()).await?{

        let mut pagination = Pagination::default();

        if !params.is_empty() {
            event!(Level::INFO, pagination = true);
            pagination = extract_pagination(params)?;
        }

        match store.get_documents(pagination.limit, pagination.offset).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e)),
        }

    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }

}

#[instrument]
pub async fn get_document(
    id: String,
    session: Session,
    store: DocumentStore,
    auth_store: AuthStore,
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying document");

    let role: Roles = Roles::CE;
    if auth_store.has_authorization(session.account_id.0, role.as_str().to_string()).await?{
        match store.get_document(id).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e)),
        }

    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
    
}


#[instrument]
pub async fn delete_document(
    id: String,
    session: Session,
    store: DocumentStore,
    auth_store: AuthStore,
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "deleting document");

    let role: Roles = Roles::ADMIN;
    if auth_store.has_authorization(session.account_id.0, role.as_str().to_string()).await?{

        match store.delete_document(id).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e)),
        }
    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
  
}