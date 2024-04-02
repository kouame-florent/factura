
use std::collections::HashMap;
use warp::http::StatusCode;
use tracing::{event, instrument, Level};

use crate::{
    repo::fournisseur::FournisseurRepo, 
    types::{fournisseur::Fournisseur, pagination::{extract_pagination, Pagination}},
    
};

#[instrument]
pub async fn add_fournisseur(
    repo: FournisseurRepo,
    fournisseur: Fournisseur,
) -> Result<impl warp::Reply, warp::Rejection>{
    match repo.add_fournisseur(fournisseur).await {
        Ok(_) => {
            Ok(warp::reply::with_status("Fournisseur added", StatusCode::OK))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn update_fournisseur(
    id: String,
    repo: FournisseurRepo,
    fournisseur: Fournisseur,
) -> Result<impl warp::Reply, warp::Rejection>{
    match repo.update_fournisseur(fournisseur, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e))
    }
}

#[instrument]
pub async fn get_fournisseurs(
    params: HashMap<String, String>,
    repo: FournisseurRepo,
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying fournisseur");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }

    match repo.get_fournisseurs(pagination.limit, pagination.offset).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn get_fournisseur(
    id: String,
    repo: FournisseurRepo
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying fournisseur");

    match repo.get_fournisseur(id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn delete_fournisseur(
    id: String,
    repo: FournisseurRepo
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying fournisseur");

    match repo.delete_fournisseur(id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}