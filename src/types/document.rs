use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::types::dossier_fournisseur::DossierFournisseurId;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Document {
    pub id: DocumentId,
    pub dossier_fournisseur_id: DossierFournisseurId,
    pub code: String,
    pub libelle: String,
    pub categorie: Categorie,
    pub date_signature: Option<NaiveDateTime>,
    pub signataire: Option<String>,
    pub montant: Option<i64>,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub updated_by: String,

   
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewDocument {
    pub dossier_fournisseur_id: DossierFournisseurId,
    pub code: String,
    pub libelle: String,
    pub categorie: Categorie,
    pub date_signature: Option<NaiveDateTime>,
    pub signataire: Option<String>,
    pub montant: Option<i64>,
    
   
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdatedDocument {
    pub dossier_fournisseur_id: DossierFournisseurId,
    pub code: String,
    pub libelle: String,
    pub categorie: Categorie,
    pub date_signature: Option<NaiveDateTime>,
    pub signataire: Option<String>,
    pub montant: Option<i64>,
    
   
}


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentId(pub String);


#[derive(sqlx::Type)]
#[sqlx(type_name = "categorie_document")]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Categorie{
    FactureProformat,
    FactureDefinitive,
    BonDeLivraison,
    BonDeCommande,

}

impl Categorie {
    
    pub fn as_str(&self) -> &'static str{
        match self {
            Categorie::BonDeCommande => "BonDeCommande",
            Categorie::BonDeLivraison => "BonDeLivraison",
            Categorie::FactureDefinitive => "FactureDefinitive",
            Categorie::FactureProformat => "FactureProformat",
        }
    }
}