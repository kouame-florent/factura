use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


use crate::dtos::fournisseur::FournisseurId;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostDossierFournisseurAnswer {
    pub id: DossierFournisseurId,
    pub fournisseur_id: FournisseurId,
    pub designation: String,
    pub date_creation: NaiveDateTime,
    pub numero_courier: String,
    pub updated_on: NaiveDateTime,
    pub updated_by: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostDossierFournisseurRequest {
    pub fournisseur_id: FournisseurId,
    pub designation: String,
    pub date_creation: NaiveDateTime,
    pub numero_courier: String,

}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PutDossierFournisseurRequest {
    pub fournisseur_id: FournisseurId,
    pub designation: String,
    pub date_creation: NaiveDateTime,
    pub numero_courier: String,

}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PutDossierFournisseurAnswer {
    pub id: DossierFournisseurId,
    pub fournisseur_id: FournisseurId,
    pub designation: String,
    pub date_creation: NaiveDateTime,
    pub numero_courier: String,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub updated_by: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetDossierFournisseurAnswer { 
    pub id: DossierFournisseurId,
    pub fournisseur_id: FournisseurId,
    pub designation: String,
    pub date_creation: NaiveDateTime,
    pub numero_courier: String,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub updated_by: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DossierFournisseurId(pub String);

