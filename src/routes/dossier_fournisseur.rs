
use std::collections::HashMap;
use tracing::{event, instrument, Level};

use crate::{
    store::{authentication::AuthStore, dossier_fournisseur::DossierFournisseurStore},
    types::{account::{Roles, Session}, dossier_fournisseur::{NewDossierFournisseur, UpdatedDossierFournisseur}, pagination::{extract_pagination, Pagination}},
    
};

#[instrument]
pub async fn add_dossier_fournisseur(
    session: Session,
    store: DossierFournisseurStore,
    auth_store: AuthStore,
    dossier_fournisseur: NewDossierFournisseur,
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
        match store.add_dossier_fournisseur(dossier_fournisseur,email).await {
            Ok(d) => {
                Ok(warp::reply::json(&d))
            }
            Err(e) => Err(warp::reject::custom(e)),
        }
    }else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
    
}

#[instrument]
pub async fn update_dossier_fournisseur(
    id: String,
    session: Session,
    store: DossierFournisseurStore,
    auth_store: AuthStore,
    dossier_fournisseur: UpdatedDossierFournisseur,
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

        match store.update_dossier_fournisseur(dossier_fournisseur, id, email).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e))
        }
    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }

   
}

#[instrument]
pub async fn get_dossiers_fournisseurs(
    session: Session,
    params: HashMap<String, String>,
    store: DossierFournisseurStore,
    auth_store: AuthStore,
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying 'dossier fournisseur'");
    
    let account_id = session.account_id;
    let role: Roles = Roles::CE;
    if auth_store.has_authorization(account_id.0, role.as_str().to_string()).await?{

        let mut pagination = Pagination::default();

        if !params.is_empty() {
            event!(Level::INFO, pagination = true);
            pagination = extract_pagination(params)?;
        }

        match store.get_dossiers_fournisseurs(pagination.limit, pagination.offset).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e)),
        }
    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }


}

#[instrument]
pub async fn get_dossier_fournisseur(
    id: String,
    session: Session,
    store: DossierFournisseurStore,
    auth_store: AuthStore,
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying 'dossier fournisseur'");

    let role: Roles = Roles::CE;
    if auth_store.has_authorization(session.account_id.0, role.as_str().to_string()).await?{
        match store.get_dossier_fournisseur(id).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e)),
        }

    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }

   
}

#[instrument]
pub async fn delete_dossier_fournisseur(
    id: String,
    session: Session,
    store: DossierFournisseurStore,
    auth_store: AuthStore,
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying 'dossier fournisseur' ");

    let role: Roles = Roles::ADMIN;
    if auth_store.has_authorization(session.account_id.0, role.as_str().to_string()).await?{
        match store.delete_dossier_fournisseur(id).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e)),
        }
    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
    

   
}