use std::{fs::{self, FileType}, io::Cursor};

use chrono::NaiveDate;
use reqwest::{header::{CONTENT_DISPOSITION, CONTENT_TYPE}, multipart, StatusCode};

use crate::dtos::{document::{PostDocumentAnswer, PostDocumentRequest}, 
    dossier_fournisseur::{PostDossierFournisseurAnswer, PostDossierFournisseurRequest},
    fournisseur::{PostFournisseurAnswer,FournisseurId, PostFournisseurRequest},
    user::PostUserRequest,
    fichier::PostFichierAnswer,
};

use factura::handle_errors;

use super::utils::{get_email, register_and_login};



pub async fn post_fichier() {
    
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
            .header("Authorization", token.0.clone())
            .json(&doc)
            .send()
            .await
            .unwrap()
            .json::<PostDocumentAnswer>()
            .await
            .unwrap();

    let file = fs::read("/home/florent/Documents/projects/project-anrmp/files/dog.png".to_string()).unwrap();
    let file_part = reqwest::multipart::Part::bytes(file)
        .file_name("dog.png")
        .mime_str("image/png")
        .unwrap();

    let doc_id_part = reqwest::multipart::Part::text(doc_res.id.to_string());

    let form = reqwest::multipart::Form::new()
        .part("document_id",doc_id_part)
        .part("file", file_part);

    let fichier_res = client
        .post("http://localhost:3030/fichiers")
        .header("Authorization", token.0)
        .multipart(form)
        .send()
        .await
        .unwrap()
        .json::<PostFichierAnswer>()
        .await
        .unwrap();

    assert_eq!(fichier_res.mime_type, "image/png");
    assert_eq!(fichier_res.file_name, "dog.png")

}

pub async fn get_fichier() {
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
            .header("Authorization", token.0.clone())
            .json(&doc)
            .send()
            .await
            .unwrap()
            .json::<PostDocumentAnswer>()
            .await
            .unwrap();

    let file = fs::read("/home/florent/Documents/projects/project-anrmp/files/dog.png".to_string()).unwrap();
    let file_part = reqwest::multipart::Part::bytes(file)
            .file_name("dog.png")
            .mime_str("image/png")
            .unwrap();
        
    let doc_id_part = reqwest::multipart::Part::text(doc_res.id.to_string());
        
    let form = reqwest::multipart::Form::new()
            .part("document_id",doc_id_part)
            .part("file", file_part);
        
    let fichier_res = client
            .post("http://localhost:3030/fichiers")
            .header("Authorization", token.0.clone())
            .multipart(form)
            .send()
            .await
            .unwrap()
            .json::<PostFichierAnswer>()
            .await
            .unwrap();


    let raw_url = "http://localhost:3030/fichiers/".to_owned();
    let id = fichier_res.id.clone();
    let get_url = format!("{raw_url}{id}");
    //let file_name = format!("{}/{}.{}", files_dir, f.data_pointer, ext);
        
    let get_res = client
            .get(get_url)
            .header("Authorization", token.0)
            .send()
            .await
            .unwrap();

   // let file_ext = get_res.headers().get(CONTENT_TYPE).unwrap();
    let disposition = get_res.headers().get(CONTENT_DISPOSITION).unwrap();
    println!("--- DISPOSITION: {:?}",disposition);
    
    let att_filename = disposition.to_str()
            .unwrap().split(";")
            .find(|v| v.contains("filename"))
            .unwrap()
            .trim()
            .split("=").find(|v| !v.contains("filename"))
            .unwrap();

            
    print!("--- ATT_FILENAME: {:?}", att_filename);


    let download_path = format!("/home/florent/Documents/projects/project-anrmp/downloads/{}",att_filename);
    
    let mut downloaded_file = std::fs::File::create(download_path).unwrap();

    let mut content = Cursor::new(get_res.bytes().await.unwrap());

    let copy_res = std::io::copy(&mut content, &mut downloaded_file).unwrap();


   // let meta = downloaded_file.metadata().unwrap();
    
    
    assert!(copy_res > 0);
        

}


fn get_file_extension(content_type: String) -> Result<String, handle_errors::Error>{

    let file_ending;
    match content_type.as_str() { 
        "application/pdf" => {
            file_ending = "pdf";
        }
        "image/png" => {
            file_ending = "png";
        }
        "image/jpeg" => {
            file_ending = "jpeg";
        }
        v => {
            eprintln!("invalid file type found: {}", v);
            return Err(handle_errors::Error::UnknownMimeType);
        }
    

    }

    Ok(file_ending.to_string())

}
