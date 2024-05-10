use sqlx::postgres::{PgPool,PgRow};
use sqlx::Row;

use handle_errors::Error;

use crate::types::document::DocumentId;
use crate::types::fichier::{
    Fichier,
    NewFichier,
};

#[derive(Debug, Clone)]
pub struct FichierStore{
    connection: PgPool,
}

impl FichierStore{

    pub async fn new(pool: PgPool) -> Self{
        FichierStore{
            connection: pool
        }

    }

    pub async fn add_fichier(
        &self,
        new_fichier: NewFichier,
        updated_by: String,
    ) -> Result<Fichier, Error> {
        match sqlx::query(
            "INSERT INTO fichier (
                    id,
                    document_id,
                    file_name,
                    file_size,
                    mime_type,
                    data_pointer,
                    updated_by
                )
                 VALUES ($1, $2, $3, $4, $5, $6, $7)
                 RETURNING *",
        )
        .bind(uuid::Uuid::new_v4().to_string()) 
        .bind(new_fichier.document_id.0)
        .bind(new_fichier.file_name)
        .bind(new_fichier.file_size)  
        .bind(new_fichier.mime_type)  
        .bind(new_fichier.data_pointer)  
        .bind(updated_by) 
        .map(|row: PgRow| Fichier {
            id: row.get("id"),
            document_id: DocumentId(row.get("document_id")),
            file_name: row.get("file_name"),
            file_size: row.get("file_size"),
            mime_type: row.get("mime_type"),
            data_pointer: row.get("data_pointer"),
            created_on: row.get("created_on"),
            updated_on: row.get("updated_on"),
            updated_by: row.get("updated_by")
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(fichier) => Ok(fichier),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }

    }


    pub async fn get_fichier(
        &self,
        id: String,
    ) -> Result<Fichier, Error> {
        match sqlx::query("SELECT * from fichier WHERE id = $1")
        .bind(id)
        .map(|row: PgRow| Fichier {
            id: row.get("id"),
            document_id: DocumentId(row.get("document_id")),
            file_name: row.get("file_name"),
            file_size: row.get("file_size"),
            mime_type: row.get("mime_type"),
            data_pointer: row.get("data_pointer"),
            created_on: row.get("created_on"),
            updated_on: row.get("updated_on"),
            updated_by: row.get("updated_by")
        })
        .fetch_one(&self.connection)
        .await {
            Ok(fichier) => Ok(fichier),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
        }
    } 
        

    }


}