use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Operation {
    pub id: OperationId,
    pub code: String,
    pub libelle: String,
    pub categorie: CategorieOperation,
    pub date: DateTime<Utc>,

}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct OperationId(pub String);


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CategorieOperation{
    ArriveeDeCourrier,
    Transmission,
    Enregistrement,
    Control,
    Engagement,
    Liquidation,
    Mandatement,
    TraitementDeDiffere,

}