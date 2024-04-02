use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use crate::types::{
    dossier_fournisseur::DossierFournisseur,
    dossier_fournisseur::DossierFournisseurId,
    fournisseur::Fournisseur,
    fournisseur::FournisseurId,
};

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct DossierFournisseurRepo{
    pub connection: PgPool,
}
  
impl DossierFournisseurRepo{
    pub async fn new(db_url: &str) -> Self {

        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await {
                Ok(pool) => pool,
                Err(e) => panic!("Couldn't establish DB connection: {}", e)
            };

        DossierFournisseurRepo{
            connection: db_pool,
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


}