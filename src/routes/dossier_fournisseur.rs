
use std::collections::HashMap;
use warp::http::StatusCode;
use tracing::{event, instrument, Level};

use crate::{
    store::dossier_fournisseur::DossierFournisseurStore,
    types::{fournisseur::Fournisseur,dossier_fournisseur::DossierFournisseur, pagination::{extract_pagination, Pagination}},
    
};

#[instrument]
pub async fn add_dossier_fournisseur(
    store: DossierFournisseurStore,
    dossier_fournisseur: DossierFournisseur,
) -> Result<impl warp::Reply, warp::Rejection>{
    match store.add_dossier_fournisseur(dossier_fournisseur).await {
        Ok(_) => {
            Ok(warp::reply::with_status("Dossier fournisseur added", StatusCode::OK))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// #[instrument]
// pub async fn update_dossier_fournisseur(
//     id: String,
//     store: Store,
//     fournisseur: Fournisseur,
// ) -> Result<impl warp::Reply, warp::Rejection>{
//     match store.update_fournisseur(fournisseur, id).await {
//         Ok(res) => Ok(warp::reply::json(&res)),
//         Err(e) => Err(warp::reject::custom(e))
//     }
// }

#[instrument]
pub async fn get_dossiers_fournisseurs(
    params: HashMap<String, String>,
    store: DossierFournisseurStore,
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying fournisseur");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }

    match store.get_dossiers_fournisseurs(pagination.limit, pagination.offset).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn get_dossier_fournisseur(
    id: String,
    store: DossierFournisseurStore,
) -> Result<impl warp::Reply, warp::Rejection>{
    event!(target: "factura", Level::INFO, "querying fournisseur");

    match store.get_dossier_fournisseur(id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// #[instrument]
// pub async fn delete_fournisseur(
//     id: String,
//     store: DossierFournisseurStore,
// ) -> Result<impl warp::Reply, warp::Rejection>{
//     event!(target: "factura", Level::INFO, "querying fournisseur");

//     match store.delete_(id).await {
//         Ok(res) => Ok(warp::reply::json(&res)),
//         Err(e) => Err(warp::reject::custom(e)),
//     }
// }