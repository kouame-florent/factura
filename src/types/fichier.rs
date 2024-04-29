

use crate::types::document::DocumentId;

pub struct Fichier{
    pub document_id: DocumentId,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub file_reference: String,
    
}