use chrono::Utc; 
use sqlx::postgres::{PgPool,PgRow};
use sqlx::Row;

use handle_errors::Error;


use crate::types::fournisseur::{NewFournisseur, UpdatedFournisseur};
use crate::types::{
    fournisseur::Fournisseur,
    fournisseur::FournisseurId,
    dossier_fournisseur::DossierFournisseur,
    dossier_fournisseur::DossierFournisseurId,
    
};

#[derive(Debug, Clone)]
pub struct FournisseurStore{
    connection: PgPool,
}

impl FournisseurStore{

    pub async fn new(pool: PgPool) -> Self{
        FournisseurStore{
            connection: pool
        }

    }

    pub async fn add_fournisseur(
        &self,
        new_fournisseur: NewFournisseur,
        updated_by: String,
    ) -> Result<Fournisseur, Error> {
        match sqlx::query(
            "INSERT INTO fournisseur (id, code, sigle, designation, telephone, email, updated_by)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)
                 RETURNING id, code, sigle, designation, telephone, email, created_on, updated_on, updated_by",
        )
        .bind(uuid::Uuid::new_v4().to_string()) 
        .bind(new_fournisseur.code)
        .bind(new_fournisseur.sigle)
        .bind(new_fournisseur.designation)  
        .bind(new_fournisseur.telephone)  
        .bind(new_fournisseur.email)  
        .bind(updated_by) 
        .map(|row: PgRow| Fournisseur {
            id: FournisseurId(row.get("id")),
            code: row.get("code"),
            sigle: row.get("sigle"),
            designation: row.get("designation"),
            telephone: row.get("telephone"),
            email: row.get("email"),
            created_on: row.get("created_on"),
            updated_on: row.get("updated_on"),
            updated_by: row.get("updated_by")
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
                telephone: row.get("telephone"),
                email: row.get("email"),
                created_on: row.get("created_on"),
                updated_on: row.get("updated_on"),
                updated_by: row.get("updated_by")
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
                telephone: row.get("telephone"),
                email: row.get("email"),
                created_on: row.get("created_on"),
                updated_on: row.get("updated_on"),
                updated_by: row.get("updated_by")
            })
            .fetch_one(&self.connection)
            .await {
                Ok(fournisseur) => Ok(fournisseur),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "{:?}", e);
                    Err(Error::DatabaseQueryError(e))
            }
        } 
            

    }


    pub async fn update_fournisseur(
        &self,
        updated_fournisseur: UpdatedFournisseur,
        fournisseur_id: String,
        updated_by: String,
    ) -> Result<Fournisseur, Error>{
        match sqlx::query(
            "UPDATE fournisseur SET code = $1, 
            sigle = $2,
            designation = $3,
            telephone = $4,
            email = $5,
            updated_on = $6,
            updated_by = $7
            WHERE id = $8
            RETURNING id, code, sigle, designation, telephone, email, created_on, updated_on, updated_by",
        ).bind(updated_fournisseur.code)
        .bind(updated_fournisseur.sigle)
        .bind(updated_fournisseur.designation)
        .bind(updated_fournisseur.telephone)
        .bind(updated_fournisseur.email)
        .bind(Utc::now().naive_utc())
        .bind(updated_by)
        .bind(fournisseur_id)
        .map(|row: PgRow| Fournisseur {
            id: FournisseurId(row.get("id")),
            code: row.get("code"),
            sigle: row.get("sigle"),
            designation: row.get("designation"),
            telephone: row.get("telephone"),
            email: row.get("email"),
            created_on: row.get("created_on"),
            updated_on: row.get("updated_on"),
            updated_by: row.get("updated_by")
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
                Err(Error::DatabaseQueryError(e))
            }
        }
    }


    pub async fn get_dossiers(
        &self,
        id: String,
    )-> Result<Vec<DossierFournisseur>, Error>{
        match sqlx::query("SELECT * from dossier_fournisseur 
                                WHERE fournisseur_id = $1")
            .bind(id)
            .map(|row: PgRow| DossierFournisseur {
                id: DossierFournisseurId(row.get("id")),
                fournisseur_id: FournisseurId(row.get("fournisseur_id")),
                designation: row.get("designation"),
                date_creation: row.get("designation"),
                numero_courier: row.get("numero_courier"),
                created_on: row.get("created_on"),
                updated_on: row.get("updated_on"),
                updated_by: row.get("updated_by"),
            })
            .fetch_all(&self.connection)
            .await {
                Ok(dossier_fournisseur) => Ok(dossier_fournisseur),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "{:?}", e);
                    Err(Error::DatabaseQueryError(e))
            }
        } 
            
    }

    pub async fn get_dossier(
        &self,
        dossier_id: String,
        fournisseur_id: String,
    )-> Result<Vec<DossierFournisseur>, Error>{
        match sqlx::query(
             "SELECT * from dossier_fournisseur 
                  WHERE id = $1 and fournisseur_id = $2")
            .bind(dossier_id)
            .bind(fournisseur_id)
            .map(|row: PgRow| DossierFournisseur {
                id: DossierFournisseurId(row.get("id")),
                fournisseur_id: FournisseurId(row.get("fournisseur_id")),
                designation: row.get("designation"),
                date_creation: row.get("date_creation"),
                numero_courier: row.get("numero_courier"),
                created_on: row.get("created_on"),
                updated_on: row.get("updated_on"),
                updated_by: row.get("updated_by"),
            })
            .fetch_all(&self.connection)
            .await {
                Ok(dossier_fournisseur) => Ok(dossier_fournisseur),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "{:?}", e);
                    Err(Error::DatabaseQueryError(e))
            }
        } 
            
    }

    

}