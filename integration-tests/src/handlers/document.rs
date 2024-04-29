use chrono::NaiveDate;

use crate::dtos::dossier_fournisseur::{PostDossierFournisseurAnswer, PostDossierFournisseurRequest};
use crate::dtos::fournisseur::{PostFournisseurAnswer,FournisseurId, PostFournisseurRequest};
use crate::dtos::user::{
    Token,
    User,
};

use crate::dtos::document::{
    PostDocumentRequest,
    PostDocumentAnswer,
};

pub async fn post_document(token: Token) {

    let f = PostFournisseurRequest {
        code: "d-01".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
    
    };

    let client = reqwest::Client::new();
    let f_resp = client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();



    let dos_f = PostDossierFournisseurRequest{
        fournisseur_id: FournisseurId(f_resp.id.0),
        designation: "Livraison d'ordinateur".to_string(),
        date_creation: NaiveDate::from_ymd_opt(2024, 04, 01).unwrap().and_hms_opt(0, 0, 0).unwrap(),
        numero_courier: "c-08-25-2024-18-23".to_string(),
    
    };

    let client = reqwest::Client::new();
    let dof_res = client
        .post("http://localhost:3030/dossiers-fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&dos_f)
        .send()
        .await
        .unwrap()
        .json::<PostDossierFournisseurAnswer>()
        .await
        .unwrap();


    let doc = PostDocumentRequest {
        code: "doc-01".to_string(),
        categorie: "BonDeCommande".to_string(),
        dossier_fournisseur_id: dof_res.id.0,
        date_signature: Some(NaiveDate::from_ymd_opt(2024, 02, 07).unwrap()
            .and_hms_opt(0, 0, 0).unwrap(),),
        libelle: "Bon de command machine outils".to_string(),
        montant: Some(1000000i64),
        signataire: Some("Diracteur SGB".to_string()),

    
    };

    let client = reqwest::Client::new();
    let doc_res = client
        .post("http://localhost:3030/documents")
        .header("Authorization", token.0)
        .json(&doc)
        .send()
        .await
        .unwrap()
        .json::<PostDocumentAnswer>()
        .await
        .unwrap();

    assert_eq!(doc_res.code, "doc-01");
    assert_eq!(doc_res.categorie, "BonDeCommande");
}