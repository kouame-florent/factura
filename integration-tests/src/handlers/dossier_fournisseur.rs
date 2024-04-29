
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
    GetDossierFournisseurAnswer,
};

use crate::dtos::fournisseur::{
    FournisseurId, PostFournisseurAnswer, PostFournisseurRequest,
};

pub async fn post_dossier_fournisseur(token: Token) {

    let f = PostFournisseurRequest {
        code: "d-01".to_string(),
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


pub async fn put_dossier_fournisseur(token: Token) {

    let f = PostFournisseurRequest {
        code: "d-02".to_string(),
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
    let post_res = client
        .post("http://localhost:3030/dossiers-fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&d)
        .send()
        .await
        .unwrap()
        .json::<PostDossierFournisseurAnswer>()
        .await
        .unwrap();


    let udp_dossier = PutDossierFournisseurRequest{
            fournisseur_id: post_res.fournisseur_id,
            designation: "Livraison d'ordinateurs portables".to_string(),
            date_creation: NaiveDate::from_ymd_opt(2024, 02, 07).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            numero_courier: "c-09-25-2024-19-10".to_string(),
        
    };

    let raw_url = "http://localhost:3030/dossiers-fournisseurs/".to_owned();
    let id = post_res.id.0.clone();
    let get_url = format!("{raw_url}{id}");

    let df_upd_res = client
        .put(get_url)
        .header("Authorization", token.0)
        .json(&udp_dossier)
        .send()
        .await
        .unwrap()
        .json::<PutDossierFournisseurAnswer>()
        .await
        .unwrap();

        assert_eq!(df_upd_res.designation,"Livraison d'ordinateurs portables");
        assert_eq!(df_upd_res.date_creation,NaiveDate::from_ymd_opt(2024, 02, 07).unwrap().and_hms_opt(0, 0, 0).unwrap());
        assert_eq!(df_upd_res.numero_courier,"c-09-25-2024-19-10");
}


pub async fn get_dossier_fournisseur_by_id(token: Token){

    let f = PostFournisseurRequest {
        code: "d-02".to_string(),
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

    let post_res = client
        .post("http://localhost:3030/dossiers-fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&d)
        .send()
        .await
        .unwrap()
        .json::<PostDossierFournisseurAnswer>()
        .await
        .unwrap();

    let raw_url = "http://localhost:3030/dossiers-fournisseurs/".to_owned();
    let id = post_res.id.0.clone();
    let get_url = format!("{raw_url}{id}");

    let get_res = client
        .get(get_url)
        .header("Authorization", token.0.clone())
        .send()
        .await
        .unwrap()
        .json::<GetDossierFournisseurAnswer>()
        .await
        .unwrap();

        assert_eq!(get_res.designation,"Livraison d'ordinateur");
        assert_eq!(get_res.numero_courier,"c-08-25-2024-18-23");

}


pub async fn list_dossiers_fournisseurs(token: Token){
    let f = PostFournisseurRequest {
        code: "d-02".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
    
    };

    let client = reqwest::Client::new();

    let f_resp: PostFournisseurAnswer = client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();


    let d1 = PostDossierFournisseurRequest{
            fournisseur_id: FournisseurId(f_resp.id.0.clone()),
            designation: "Livraison d'ordinateur".to_string(),
            date_creation: NaiveDate::from_ymd_opt(2024, 04, 01).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            numero_courier: "c-08-25-2024-18-23".to_string(),
        
        };

    let d2 = PostDossierFournisseurRequest{
            fournisseur_id: FournisseurId(f_resp.id.0.clone()),
            designation: "Achat de véhicules".to_string(),
            date_creation: NaiveDate::from_ymd_opt(2022, 07, 01).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            numero_courier: "cfao-2022-07".to_string(),
            
        };

    let d3 = PostDossierFournisseurRequest{
            fournisseur_id: FournisseurId(f_resp.id.0),
            designation: "reparation de climatiseur".to_string(),
            date_creation: NaiveDate::from_ymd_opt(2023, 10, 01).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            numero_courier: "artis-2023-07".to_string(),
        
        };
    
    let client = reqwest::Client::new();
    
    let _ = client
            .post("http://localhost:3030/dossiers-fournisseurs")
            .header("Authorization", token.0.clone())
            .json(&d1)
            .send()
            .await
            .unwrap()
            .json::<PostDossierFournisseurAnswer>()
            .await
            .unwrap();

    let _ = client
            .post("http://localhost:3030/dossiers-fournisseurs")
            .header("Authorization", token.0.clone())
            .json(&d2)
            .send()
            .await
            .unwrap()
            .json::<PostDossierFournisseurAnswer>()
            .await
            .unwrap();

    let _ = client
            .post("http://localhost:3030/dossiers-fournisseurs")
            .header("Authorization", token.0.clone())
            .json(&d3)
            .send()
            .await
            .unwrap()
            .json::<PostDossierFournisseurAnswer>()
            .await
            .unwrap();



    let res = client
            .get("http://localhost:3030/dossiers-fournisseurs?limit=2&offset=0")
            .header("Authorization", token.0.clone())
            .send()
            .await
            .unwrap()
            .json::<Vec<GetDossierFournisseurAnswer>>()
            .await
            .unwrap();
    
    assert_eq!(res.len(),2);
    
    let res = client
            .get("http://localhost:3030/dossiers-fournisseurs")
            .header("Authorization", token.0.clone())
            .send()
            .await
            .unwrap()
            .json::<Vec<GetDossierFournisseurAnswer>>()
            .await
            .unwrap();
    
    assert_eq!(res.len(),6);


}