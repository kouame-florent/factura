use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::dossier_fournisseur::DossierFournisseurId;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Operation {
    pub id: OperationId,
    pub dossier_fournisseur_id: DossierFournisseurId,
    pub code: String,
    pub libelle: String,
    pub categorie: CategorieOperation,
    pub date: DateTime<Utc>,
    pub resultat: Option<Resultat>,
    pub observation: Option<String>,
    pub motivation: Option<String>,
    

}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct OperationId(pub String);


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CategorieOperation{
    ArriveeDeCourrier,
    Transmission,
    Enregistrement,
    ControlFactureProforma,
    ControlFactureDefinitive,
    ControlBonDeCommande,
    Engagement,
    Liquidation,
    Mandatement,
    TraitementDeDiffere,

}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Resultat{
   Valide,
   Differe,
   Rejete,

}