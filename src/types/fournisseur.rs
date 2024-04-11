use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Fournisseur {
    pub id: FournisseurId,
    pub code: String,
    pub sigle: String,
    pub designation: String,
    pub telephone: String,
    pub email: String,
    pub updated_by: String,

}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FournisseurId(pub String);
