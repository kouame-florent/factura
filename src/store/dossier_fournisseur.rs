use sqlx::postgres::{PgPool, PgPoolOptions,PgRow};
use sqlx::Row;

use crate::error::Error;


use crate::types::{
    fournisseur::Fournisseur,
    fournisseur::FournisseurId,
    dossier_fournisseur::DossierFournisseur,
    dossier_fournisseur::DossierFournisseurId,
    
};

#[derive(Debug, Clone)]
pub struct DossierFournisseurStore{
    connection: PgPool,
}

impl DossierFournisseurStore{

    pub async fn new(pool: PgPool) -> Self{
        DossierFournisseurStore{
            connection: pool
        }

    }

    pub async fn add_dossier_fournisseur(
        &self,
        new_dossier_fournisseur: DossierFournisseur,
    ) -> Result<DossierFournisseur, Error>{
        match sqlx::query(
            "INSERT INTO dossier_fournisseur (id, fournisseur_id, designation)
                 VALUES ($1, $2, $3)
                 RETURNING id, fournisseur_id, designation",
        )
        .bind(new_dossier_fournisseur.id.0) 
        .bind(new_dossier_fournisseur.fournisseur_id.0) 
        .bind(new_dossier_fournisseur.designation)
        .map(|row: PgRow| DossierFournisseur {
            id: DossierFournisseurId(row.get("id")),
            fournisseur_id: FournisseurId(row.get("fournisseur_id")),
            designation: row.get("designation"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(dossier_fournisseur) => Ok(dossier_fournisseur),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn get_dossiers_fournisseurs(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<DossierFournisseur>, Error>{
        match sqlx::query("SELECT * from dossier_fournisseur LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| DossierFournisseur {
                id: DossierFournisseurId(row.get("id")),
                fournisseur_id: FournisseurId(row.get("fournisseur_id")),
                designation: row.get("designation"),
            })
            .fetch_all(&self.connection)
            .await {
                Ok(questions) => Ok(questions),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "{:?}", e);
                    Err(Error::DatabaseQueryError)
            }
        } 
            
        
    }

    pub async fn get_dossier_fournisseur(
        &self,
        id: String,
    ) -> Result<DossierFournisseur, Error>{
        match sqlx::query("SELECT * from dossier_fournisseur WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| DossierFournisseur {
                id: DossierFournisseurId(row.get("id")),
                fournisseur_id: FournisseurId(row.get("fournisseur_id")),
                designation: row.get("designation"),
            })
            .fetch_one(&self.connection)
            .await {
                Ok(dossier_fournisseur) => Ok(dossier_fournisseur),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "{:?}", e);
                    Err(Error::DatabaseQueryError)
            }
        } 
            

    }

    //return all 'dossiers' of one 'fournisseur'
    pub async fn get_all_dossiers(
        &self,
        fournisseur_id: String,
    ) -> Result<Vec<DossierFournisseur>, Error>{
        match sqlx::query("SELECT * from dossier_fournisseur WHERE fournisseur_id = $1")
            .bind(fournisseur_id)
            .map(|row: PgRow| DossierFournisseur {
                id: DossierFournisseurId(row.get("id")),
                fournisseur_id: FournisseurId(row.get("fournisseur_id")),
                designation: row.get("designation"),
            })
            .fetch_all(&self.connection)
            .await {
                Ok(dossier_fournisseur) => Ok(dossier_fournisseur),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "{:?}", e);
                    Err(Error::DatabaseQueryError)
            }
        } 
    }
}