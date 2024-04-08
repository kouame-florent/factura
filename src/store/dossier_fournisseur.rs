use sqlx::postgres::{PgPool, PgPoolOptions,PgRow};
use sqlx::Row;

use handle_errors::Error;


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
            "INSERT INTO dossier_fournisseur (id, fournisseur_id, designation, date_creation, numero_courier)
                 VALUES ($1, $2, $3, $4, $5)
                 RETURNING id, fournisseur_id, designation, date_creation, numero_courier",
        )
        .bind(new_dossier_fournisseur.id.0) 
        .bind(new_dossier_fournisseur.fournisseur_id.0) 
        .bind(new_dossier_fournisseur.designation)
        .bind(new_dossier_fournisseur.date_creation)
        .bind(new_dossier_fournisseur.numero_courier)
        .map(|row: PgRow| DossierFournisseur {
            id: DossierFournisseurId(row.get("id")),
            fournisseur_id: FournisseurId(row.get("fournisseur_id")),
            designation: row.get("designation"),
            date_creation: row.get("date_creation"),
            numero_courier: row.get("numero_courier"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(dossier_fournisseur) => Ok(dossier_fournisseur),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
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
                date_creation: row.get("date_creation"),
                numero_courier: row.get("numero_courier"),
            })
            .fetch_all(&self.connection)
            .await {
                Ok(questions) => Ok(questions),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "{:?}", e);
                    Err(Error::DatabaseQueryError(e))
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
                date_creation: row.get("date_creation"),
                numero_courier: row.get("numero_courier"),
            })
            .fetch_one(&self.connection)
            .await {
                Ok(dossier_fournisseur) => Ok(dossier_fournisseur),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "{:?}", e);
                    Err(Error::DatabaseQueryError(e))
            }
        } 
            

    }

    
    pub async fn update_dossier_fournisseur(
        &self,
        dossier_fournisseur: DossierFournisseur,
        dossier_fournisseur_id: String,
    )-> Result<DossierFournisseur, Error>{
        match sqlx::query(
            "UPDATE dossier_fournisseur SET designation = $1, numero_courier = $2
            WHERE id = $3
            RETURNING id, fournisseur_id, designation, date_creation, numero_courier",
        ).bind(dossier_fournisseur.designation)
        .bind(dossier_fournisseur.date_creation)
        .bind(dossier_fournisseur.numero_courier)
        .bind(dossier_fournisseur_id)
        .map(|row: PgRow| DossierFournisseur {
            id: DossierFournisseurId(row.get("id")),
            fournisseur_id: FournisseurId(row.get("fournisseur_id")),
            designation: row.get("designation"),
            date_creation: row.get("date_creation"),
            numero_courier: row.get("numero_courier"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(fournisseur) => Ok(fournisseur),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn delete_dossier_fournisseur(
        &self,
        dossier_fournisseur_id: String,
    ) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM dossier_fournisseur WHERE id = $1")
            .bind(dossier_fournisseur_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }


}