use sqlx::postgres::{PgPool,PgRow};
use sqlx::Row;

use handle_errors::Error;

use crate::types::document::{Document, DocumentId, NewDocument};
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

}