use chrono::Utc;
use sqlx::postgres::{PgPool,PgRow};
use sqlx::Row;

use handle_errors::Error;

use crate::types::document::{Document, DocumentId, NewDocument, UpdatedDocument};
use crate::types::dossier_fournisseur::DossierFournisseurId;


#[derive(Debug, Clone)]
pub struct DocumentStore{
    connection: PgPool,
}


impl DocumentStore{

    pub async fn new(pool: PgPool) -> Self{
        DocumentStore{
            connection: pool,
        }
    }

    pub async fn add_document(
        &self,
        new_document: NewDocument,
        updated_by: String,
    ) -> Result<Document, Error>{
        match sqlx::query(
            "INSERT INTO document (id, dossier_fournisseur_id, code, libelle, categorie, date_signature, signataire, montant, updated_by)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                 RETURNING *",
        )
        .bind(uuid::Uuid::new_v4().to_string()) 
        .bind(new_document.dossier_fournisseur_id.0)
        .bind(new_document.code)
        .bind(new_document.libelle)  
        .bind(new_document.categorie)  
        .bind(new_document.date_signature)  
        .bind(new_document.signataire)
        .bind(new_document.montant)
        .bind(updated_by) 
        .map(|row: PgRow| Document {
            id: DocumentId(row.get("id")),
            dossier_fournisseur_id: DossierFournisseurId(row.get("dossier_fournisseur_id")),
            code: row.get("code"),
            libelle: row.get("libelle"),
            categorie: row.get("categorie"),
            date_signature: row.get("date_signature"),
            signataire: row.get("signataire"),
            montant: row.get("montant"),
            created_on: row.get("created_on"),
            updated_on: row.get("updated_on"),
            updated_by: row.get("updated_by")
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(document) => Ok(document),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }


    pub async fn update_document(
        &self,
        updated_document: UpdatedDocument,
        document_id: String,
        updated_by: String,
    ) -> Result<Document, Error>{
        match sqlx::query(
            "UPDATE document SET 
            dossier_fournisseur_id = $1,
            code = $2,
            libelle = $3,
            categorie = $4,
            date_signature = $5,
            signataire = $6,
            montant = $7,
            updated_on = $8,
            updated_by = $9
            WHERE id = $10
            RETURNING *",
        ).bind(updated_document.dossier_fournisseur_id.0)
        .bind(updated_document.code)
        .bind(updated_document.libelle)
        .bind(updated_document.categorie)
        .bind(updated_document.date_signature)
        .bind(updated_document.signataire)
        .bind(updated_document.montant)
        .bind(Utc::now().naive_utc())
        .bind(updated_by)
        .bind(document_id)
        .map(|row: PgRow| Document {
            id: DocumentId(row.get("id")),
            dossier_fournisseur_id: DossierFournisseurId(row.get("dossier_fournisseur_id")),
            code: row.get("code"),
            libelle: row.get("libelle"),
            categorie: row.get("categorie"),
            date_signature: row.get("date_signature"),
            signataire: row.get("signataire"),
            montant: row.get("montant"),
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

    pub async fn get_documents(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Document>, Error>{
        match sqlx::query("SELECT * from document ORDER BY created_on ASC LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Document {
                id: DocumentId(row.get("id")),
                dossier_fournisseur_id: DossierFournisseurId(row.get("dossier_fournisseur_id")),
                code: row.get("code"),
                libelle: row.get("libelle"),
                categorie: row.get("categorie"),
                date_signature: row.get("date_signature"),
                signataire: row.get("signataire"),
                montant: row.get("montant"),
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


    pub async fn get_document(
        &self,
        id: String,
    ) -> Result<Document, Error>{
        match sqlx::query("SELECT * from document WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| Document {
                id: DocumentId(row.get("id")),
                dossier_fournisseur_id: DossierFournisseurId(row.get("dossier_fournisseur_id")),
                code: row.get("code"),
                libelle: row.get("libelle"),
                categorie: row.get("categorie"),
                date_signature: row.get("date_signature"),
                signataire: row.get("signataire"),
                montant: row.get("montant"),
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

    pub async fn delete_document(
        &self,
        document_id: String,
    ) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM document WHERE id = $1")
            .bind(document_id)
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


