use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use crate::types::{
    fournisseur::Fournisseur,
    fournisseur::FournisseurId,
    
};

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct FournisseurRepo{
    pub connection: PgPool,
}
  
impl FournisseurRepo{
    pub async fn new(db_url: &str) -> Self {

        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await {
                Ok(pool) => pool,
                Err(e) => panic!("Couldn't establish DB connection: {}", e)
            };

        FournisseurRepo{
            connection: db_pool,
        }
    }


    pub async fn get_fournisseurs(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Fournisseur>, Error>{
        match sqlx::query("SELECT * from fournisseur LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Fournisseur {
                id: FournisseurId(row.get("id")),
                code: row.get("code"),
                sigle: row.get("sigle"),
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

    pub async fn get_fournisseur(
        &self,
        id: String,
    ) -> Result<Fournisseur, Error>{
        match sqlx::query("SELECT * from fournisseur WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| Fournisseur {
                id: FournisseurId(row.get("id")),
                code: row.get("code"),
                sigle: row.get("sigle"),
                designation: row.get("designation"),
            })
            .fetch_one(&self.connection)
            .await {
                Ok(fournisseur) => Ok(fournisseur),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "{:?}", e);
                    Err(Error::DatabaseQueryError)
            }
        } 
            

    }

    pub async fn add_fournisseur(
        &self,
        new_fournisseur: Fournisseur,
    ) -> Result<Fournisseur, Error> {
        match sqlx::query(
            "INSERT INTO fournisseur (id, code, sigle, designation)
                 VALUES ($1, $2, $3, $4)
                 RETURNING id, code, sigle, designation",
        )
        .bind(new_fournisseur.id.0) 
        .bind(new_fournisseur.code)
        .bind(new_fournisseur.sigle)
        .bind(new_fournisseur.designation)  
        .map(|row: PgRow| Fournisseur {
            id: FournisseurId(row.get("id")),
            code: row.get("code"),
            sigle: row.get("sigle"),
            designation: row.get("designation"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn update_fournisseur(
        &self,
        fournisseur: Fournisseur,
        fournisseur_id: String,
    ) -> Result<Fournisseur, Error>{
        match sqlx::query(
            "UPDATE fournisseur SET code = $1, sigle = $2, designation = $3
            WHERE id = $4
            RETURNING id, code, sigle, designation",
        ).bind(fournisseur.code)
        .bind(fournisseur.sigle)
        .bind(fournisseur.designation)
        .bind(fournisseur_id)
        .map(|row: PgRow| Fournisseur {
            id: FournisseurId(row.get("id")),
            code: row.get("code"),
            sigle: row.get("sigle"),
            designation: row.get("designation"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(fournisseur) => Ok(fournisseur),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn delete_fournisseur(
        &self,
        fournisseur_id: String,
    ) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM fournisseur WHERE id = $1")
            .bind(fournisseur_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

}