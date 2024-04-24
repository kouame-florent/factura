use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostFournisseurRequest{
    pub code: String,
    pub sigle: String,
    pub designation: String,
    pub telephone: String,
    pub email: String,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostFournisseurAnswer{
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


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutFournisseurAnswer{
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutFournisseurRequest{
    pub code: String,
    pub sigle: String,
    pub designation: String,
    pub telephone: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FournisseurId(pub String);


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetFournisseurAnswer{
    pub id: String,
    pub code: String,
    pub sigle: String,
    pub designation: String,
    pub telephone: String,
    pub email: String,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub updated_by: String,
}

