
use std::collections::HashMap;
use tracing::{event, instrument, Level};

use crate::{
    store::fournissueur::FournisseurStore, types::{
         fournisseur::{NewFournisseur, UpdatedFournisseur}, pagination::{extract_pagination, 
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
    fournisseur: NewFournisseur,
) -> Result<impl warp::Reply, warp::Rejection>{
    let account_id = session.account_id;
    let email = match auth_store.get_email(account_id.0.clone()).await {  
        Ok(email) => {
            email
        },
        Err(_) => {
            return Err(warp::reject::custom(handle_errors::Error::Unauthorized));
        }
    };
    let admin: Roles = Roles::ADMIN;
    if auth_store.has_authorization(account_id.0, admin.as_str().to_string()).await?{
        match store.add_fournisseur(fournisseur, email.to_string()).await {
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
pub async fn update_fournisseur(
    id: String,
    session: Session,
    store: FournisseurStore,
    auth_store: AuthStore,
    updated_fournisseur: UpdatedFournisseur,
) -> Result<impl warp::Reply, warp::Rejection>{
    let account_id = session.account_id;
    let email = match auth_store.get_email(account_id.0.clone()).await {  
        Ok(email) => {
            email
        },
        Err(_) => {
            return Err(warp::reject::custom(handle_errors::Error::Unauthorized));
        }
    };
    let admin: Roles = Roles::ADMIN;
    if auth_store.has_authorization(account_id.0, admin.as_str().to_string()).await?{

        match store.update_fournisseur(updated_fournisseur, id, email).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e))
        }
    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }

  
}

#[instrument]
pub async fn get_fournisseurs(
    session: Session,
    params: HashMap<String, String>,
    store: FournisseurStore,
    auth_store: AuthStore,
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying fournisseur");

    let account_id = session.account_id;
    let role: Roles = Roles::CE;
    if auth_store.has_authorization(account_id.0, role.as_str().to_string()).await?{

        let mut pagination = Pagination::default();

        if !params.is_empty() {
            event!(Level::INFO, pagination = true);
            pagination = extract_pagination(params)?;
        }

        match store.get_fournisseurs(pagination.limit, pagination.offset).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e)),
        }

    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }

}

#[instrument]
pub async fn get_fournisseur(
    id: String,
    session: Session,
    store: FournisseurStore,
    auth_store: AuthStore,
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying fournisseur");

    let role: Roles = Roles::CE;
    if auth_store.has_authorization(session.account_id.0, role.as_str().to_string()).await?{
        match store.get_fournisseur(id).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e)),
        }

    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
    
}


#[instrument]
pub async fn delete_fournisseur(
    id: String,
    session: Session,
    store: FournisseurStore,
    auth_store: AuthStore,
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying fournisseur");

    let role: Roles = Roles::ADMIN;
    if auth_store.has_authorization(session.account_id.0, role.as_str().to_string()).await?{

        match store.delete_fournisseur(id).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e)),
        }
    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
  
}

#[instrument]
pub async fn get_dossiers(
    id: String,
    session: Session,
    store: FournisseurStore,
    auth_store: AuthStore,
)-> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying dossier fournisseur");

    let role: Roles = Roles::CE;
    if auth_store.has_authorization(session.account_id.0, role.as_str().to_string()).await?{
        match store.get_dossiers(id).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e)),
        }

    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
   

}

#[instrument]
pub async fn get_dossier(
    fournisseur_id: String,
    dossier_id: String,
    session: Session,
    store: FournisseurStore,
    auth_store: AuthStore,
)-> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying dossier fournisseur");

    let role: Roles = Roles::CE;
    if auth_store.has_authorization(session.account_id.0, role.as_str().to_string()).await?{
        match store.get_dossier(dossier_id,fournisseur_id).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e)),
        }

    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
    
}
