use chrono::NaiveDate;


use crate::dtos::dossier_fournisseur::{PostDossierFournisseurAnswer, PostDossierFournisseurRequest};
use crate::dtos::fournisseur::{PostFournisseurAnswer,FournisseurId, PostFournisseurRequest};
use crate::dtos::user::PostUserRequest;

use crate::dtos::document::{
    GetDocumentAnswer, PostDocumentAnswer, PostDocumentRequest, PutDocumentAnswer, PutDocumentRequest
};

use crate::handlers::utils::{get_email, register_and_login};

pub async fn post_document() {

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


pub async fn update_document(){

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
        let id = post_res.id;
        let put_url = format!("{raw_url}{id}");

        let doc_up = PutDocumentRequest{
                code: "doc-0041".to_string(),
                categorie: "FactureProformat".to_string(),
                dossier_fournisseur_id: post_res.dossier_fournisseur_id,
                date_signature: Some(NaiveDate::from_ymd_opt(2024, 02, 07).unwrap()
                    .and_hms_opt(0, 0, 0).unwrap(),),
                libelle: "Facture machine outils".to_string(),
                montant: Some(10100000i64),
                signataire: Some("Directeur SGB".to_string()),
            };

        let put_res = client
            .put(put_url)
            .header("Authorization", token.0)
            .json(&doc_up)
            .send()
            .await
            .unwrap()
            .json::<PutDocumentAnswer>()
            .await
            .unwrap();

        assert_eq!(put_res.code,"doc-0041");
        assert_eq!(put_res.categorie,"FactureProformat");
        assert_eq!(put_res.libelle,"Facture machine outils");
        assert_eq!(put_res.montant,Some(10100000i64));
        assert_eq!(put_res.signataire,Some("Directeur SGB".to_string()));
        

}


pub async fn get_document_by_id(){

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

   // println!("------ Get document by id: {:?}",get_url.clone());

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

pub async fn list_documents(){

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

        assert!(list_res.len() >= 2);
}


pub async fn delete_document() {
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
    let delete_url = format!("{raw_url}{id}");

    let del_resp = client
        .delete(delete_url)
        .header("Authorization", token.0.clone())
        .send()
        .await
        .unwrap()
        .json::<bool>()
        .await
        .unwrap();

    assert_eq!(del_resp,true);

}