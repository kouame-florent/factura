use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostDocumentRequest {
    pub dossier_fournisseur_id: String,
    pub code: String,
    pub libelle: String,
    pub categorie: String,
    pub date_signature: Option<NaiveDateTime>,
    pub signataire: Option<String>,
    pub montant: Option<i64>,
       
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostDocumentAnswer {
    pub id: String,
    pub dossier_fournisseur_id: String,
    pub code: String,
    pub libelle: String,
    pub categorie: String,
    pub date_signature: Option<NaiveDateTime>,
    pub signataire: Option<String>,
    pub montant: Option<i64>,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub updated_by: String,

   
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetDocumentAnswer {
    pub id: String,
    pub dossier_fournisseur_id: String,
    pub code: String,
    pub libelle: String,
    pub categorie: String,
    pub date_signature: Option<NaiveDateTime>,
    pub signataire: Option<String>,
    pub montant: Option<i64>,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub updated_by: String,

   
}


