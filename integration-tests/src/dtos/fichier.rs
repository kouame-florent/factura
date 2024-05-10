use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostFichierAnswer {
    pub id: String,
    pub document_id: String,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub data_pointer: String,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub updated_by: String,
  
}