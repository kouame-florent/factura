
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::types::document::DocumentId;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Fichier{
    pub id: String,
    pub document_id: DocumentId,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub data_pointer: String,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub updated_by: String,

    
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewFichier{
    pub document_id: DocumentId,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub data_pointer: String,
    
}