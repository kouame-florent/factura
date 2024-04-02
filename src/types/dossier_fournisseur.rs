use serde::{Deserialize, Serialize}; 

use crate::types::fournisseur::FournisseurId;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DossierFournisseur {
    pub id: DossierFournisseurId,
    pub fournisseur_id: FournisseurId,
    pub designation: String,

}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DossierFournisseurId(pub String);

