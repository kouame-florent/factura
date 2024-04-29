use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Fournisseur {
    pub id: FournisseurId,
    pub code: String,
    pub sigle: String,
    pub designation: String,
    pub telephone: String,
    pub email: String,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub updated_by: String,

}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FournisseurId(pub String);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewFournisseur {
    pub code: String,
    pub sigle: String,
    pub designation: String,
    pub telephone: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdatedFournisseur {
    pub code: String,
    pub sigle: String,
    pub designation: String,
    pub telephone: String,
    pub email: String,
}
