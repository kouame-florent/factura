
use chrono::{NaiveDate, NaiveDateTime};

use crate::dtos::user::{
    Token,
    User,
};

use crate::dtos::dossier_fournisseur::{
    PostDossierFournisseurRequest,
    PostDossierFournisseurAnswer,
    PutDossierFournisseurRequest,
    PutDossierFournisseurAnswer,
};

use crate::dtos::fournisseur::{
    FournisseurId, GetFournisseurAnswer, PostFournisseurAnswer, PostFournisseurRequest, PutFournisseurAnswer, PutFournisseurRequest
};

pub async fn post_dossier_fournisseur(token: Token) {

    let f = PostFournisseurRequest {
        code: "f-01".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
    
    };

    let client = reqwest::Client::new();
    let f = client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();



    let d = PostDossierFournisseurRequest{
        fournisseur_id: FournisseurId(f.id.0),
        designation: "Livraison d'ordinateur".to_string(),
        date_creation: NaiveDate::from_ymd_opt(2024, 04, 01).unwrap().and_hms_opt(0, 0, 0).unwrap(),
        numero_courier: "c-08-25-2024-18-23".to_string(),
    
    };

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/dossiers-fournisseurs")
        .header("Authorization", token.0)
        .json(&d)
        .send()
        .await
        .unwrap()
        .json::<PostDossierFournisseurAnswer>()
        .await
        .unwrap();

    assert_eq!(res.numero_courier, "c-08-25-2024-18-23");
    
}
