
use std::collections::HashMap;
use warp::http::StatusCode;
use tracing::{event, instrument, Level};

use crate::{
    store::fournissueur::FournisseurStore, types::{
        account, fournisseur::Fournisseur, pagination::{extract_pagination, 
        Pagination}
    }
    
    
};
use crate::types::account::{Session,Roles};
use crate::store::authentication::AuthStore;

#[instrument]
pub async fn add_fournisseur(
    session: Session,
    store: FournisseurStore,
    auth_store: AuthStore,
    fournisseur: Fournisseur,
) -> Result<impl warp::Reply, warp::Rejection>{

    let account_id = session.account_id;
    let admin: Roles = Roles::ADMIN;
    if auth_store.has_authorization(account_id.0, admin.as_str().to_string()).await?{
        match store.add_fournisseur(fournisseur).await {
            Ok(_) => {
                Ok(warp::reply::with_status("Fournisseur added", StatusCode::OK))
            }
            Err(e) => Err(warp::reject::custom(e)),
        }

    }else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
    
}

#[instrument]
pub async fn update_fournisseur(
    id: String,
    session: Session,
    store: FournisseurStore,
    auth_store: AuthStore,
    fournisseur: Fournisseur,
) -> Result<impl warp::Reply, warp::Rejection>{
    let account_id = session.account_id;
    let admin: Roles = Roles::ADMIN;
    if auth_store.has_authorization(account_id.0, admin.as_str().to_string()).await?{
          
        match store.update_fournisseur(fournisseur, id).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e))
        }
    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }

  
}

#[instrument]
pub async fn get_fournisseurs(
    params: HashMap<String, String>,
    store: FournisseurStore,
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying fournisseur");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }

    match store.get_fournisseurs(pagination.limit, pagination.offset).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn get_fournisseur(
    id: String,
    store: FournisseurStore,
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying fournisseur");

    match store.get_fournisseur(id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn get_dossiers(
    id: String,
    store: FournisseurStore,
)-> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying dossier fournisseur");
    match store.get_dossiers(id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }

}

#[instrument]
pub async fn get_dossier(
    fournisseur_id: String,
    dossier_id: String,
    store: FournisseurStore,
)-> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying dossier fournisseur");
    match store.get_dossier(dossier_id,fournisseur_id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }

}

#[instrument]
pub async fn delete_fournisseur(
    id: String,
    store: FournisseurStore,
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying fournisseur");

    match store.delete_fournisseur(id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}