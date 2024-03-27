use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Fournisseur {
    id: FournisseurId,
    code: String,
    sigle: String,
    designation: String,

}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FournisseurId(pub String);