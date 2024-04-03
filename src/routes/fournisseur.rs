
use std::collections::HashMap;
use warp::http::StatusCode;
use tracing::{event, instrument, Level};

use crate::{
    types::{fournisseur::Fournisseur, pagination::{extract_pagination, Pagination}},
    store::fournissueur::FournisseurStore,
    
};

#[instrument]
pub async fn add_fournisseur(
    store: FournisseurStore,
    fournisseur: Fournisseur,
) -> Result<impl warp::Reply, warp::Rejection>{
    match store.add_fournisseur(fournisseur).await {
        Ok(_) => {
            Ok(warp::reply::with_status("Fournisseur added", StatusCode::OK))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn update_fournisseur(
    id: String,
    store: FournisseurStore,
    fournisseur: Fournisseur,
) -> Result<impl warp::Reply, warp::Rejection>{
    match store.update_fournisseur(fournisseur, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e))
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