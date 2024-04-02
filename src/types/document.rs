use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Document {
    pub id: DocumentId,
    pub code: String,
    pub libelle: String,
    pub date_signataire: DateTime,
    pub signataire: String,
    pub montant: i64,
    pub dossier_fournisseur_id: DosssierFournisseurId,
    pub designation: String,
    pub reference_fichier: String,

}


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentId(pub String);

