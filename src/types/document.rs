use serde::{Deserialize, Serialize};
use chrono::prelude::*;

use crate::types::dossier_fournisseur::DossierFournisseurId;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Document {
    pub id: DocumentId,
    pub code: String,
    pub libelle: String,
    pub date_signature: u32,
    pub signataire: String,
    pub montant: i64,
    pub dossier_fournisseur_id: DossierFournisseurId,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub file_reference: String,
    
}


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentId(pub String);

