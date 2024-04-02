use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DosssierFournisseur {
    pub id: DosssierFournisseurId,
    pub fournisseur_id: FournisseurId,
    pub designation: String,

}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DosssierFournisseurId(pub String);

