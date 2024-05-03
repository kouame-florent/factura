use chrono::NaiveDate;
use factura::config::Config;

use crate::dtos::dossier_fournisseur::{PostDossierFournisseurAnswer, PostDossierFournisseurRequest};
use crate::dtos::fournisseur::{PostFournisseurAnswer,FournisseurId, PostFournisseurRequest};
use crate::dtos::user::{
    Token,
    PostUserRequest,
};

use crate::dtos::document::{
    GetDocumentAnswer, PostDocumentAnswer, PostDocumentRequest
};

use crate::handlers::utils::{create_db, drop_db, get_email, register_and_login};

pub async fn post_document(config: &Config) {

    drop_db(config).await.unwrap();
    create_db(config).await.unwrap();

    let user = PostUserRequest {
        email: get_email(),
        password: "password".to_string(),
        roles: "ADMIN,CE,DAFP".to_string(),
    };

    let token = register_and_login(&user).await;

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

pub async fn get_document_by_id(config: &Config){

    drop_db(config).await.unwrap();
    create_db(config).await.unwrap();

    let user = PostUserRequest {

        email: get_email(),
        password: "password".to_string(),
        roles: "ADMIN,CE,DAFP".to_string(),
    };

    let token = register_and_login(&user).await;

    let f = PostFournisseurRequest {
        code: "d-04".to_string(),
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

    let post_res = client
        .post("http://localhost:3030/documents")
        .header("Authorization", token.0.clone())
        .json(&doc)
        .send()
        .await
        .unwrap()
        .json::<PostDocumentAnswer>()
        .await
        .unwrap();

    let raw_url = "http://localhost:3030/documents/".to_owned();
    let id = post_res.id.clone();
    let get_url = format!("{raw_url}{id}");

    let get_res = client
        .get(get_url)
        .header("Authorization", token.0.clone())
        .send()
        .await
        .unwrap()
        .json::<GetDocumentAnswer>()
        .await
        .unwrap();

    assert_eq!(post_res.id, get_res.id)

}

pub async fn list_documents(config: &Config){

    drop_db(config).await.unwrap();
    create_db(config).await.unwrap();

    let user = PostUserRequest {
        email: "test@email.com".to_string(),
        password: "password".to_string(),
        roles: "ADMIN,CE,DAFP".to_string(),
    };

    let token = register_and_login(&user).await;

    let f = PostFournisseurRequest {
        code: "d-04".to_string(),
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
        code: "doc-001".to_string(),
        categorie: "BonDeCommande".to_string(),
        dossier_fournisseur_id: dof_res.id.0.clone(),
        date_signature: Some(NaiveDate::from_ymd_opt(2024, 02, 07).unwrap()
            .and_hms_opt(0, 0, 0).unwrap(),),
        libelle: "Bon de command machine outils".to_string(),
        montant: Some(1000000i64),
        signataire: Some("Diracteur SGB".to_string()),

    
    };

    let doc1 = PostDocumentRequest {
        code: "doc-002".to_string(),
        categorie: "BonDeCommande".to_string(),
        dossier_fournisseur_id: dof_res.id.0,
        date_signature: Some(NaiveDate::from_ymd_opt(2024, 02, 07).unwrap()
            .and_hms_opt(0, 0, 0).unwrap(),),
        libelle: "Bon de command machine outils".to_string(),
        montant: Some(1000000i64),
        signataire: Some("Diracteur SGB".to_string()),

    
    };

    let _ = client
        .post("http://localhost:3030/documents")
        .header("Authorization", token.0.clone())
        .json(&doc)
        .send()
        .await
        .unwrap()
        .json::<PostDocumentAnswer>()
        .await
        .unwrap();

    let _ = client
        .post("http://localhost:3030/documents")
        .header("Authorization", token.0.clone())
        .json(&doc1)
        .send()
        .await
        .unwrap()
        .json::<PostDocumentAnswer>()
        .await
        .unwrap();


    let list_res = client
        .get("http://localhost:3030/documents/")
        .header("Authorization", token.0.clone())
        .send()
        .await
        .unwrap()
        .json::<Vec<GetDocumentAnswer>>()
        .await
        .unwrap();

        assert_eq!(list_res.len(),2);
}