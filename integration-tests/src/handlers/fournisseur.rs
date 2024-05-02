use factura::config::Config;
use factura::handle_errors;
use reqwest::Error;
use serde_json::Value;

use crate::dtos::dossier_fournisseur::PostDossierFournisseurRequest;
use crate::dtos::user::{
    Token,
    PostUserRequest,
};
use crate::dtos::fournisseur::{
    PostFournisseurAnswer,
    PostFournisseurRequest,
    PutFournisseurRequest,
    PutFournisseurAnswer,
    GetFournisseurAnswer,
};
use crate::handlers::test_init::{init_db, init_user};


pub async fn post_fournisseur(config: &Config) {

    init_db(config).unwrap(); 

    let user = PostUserRequest {
        email: "test@email.com".to_string(),
        password: "password".to_string(),
        roles: "ADMIN,CE,DAFP".to_string(),
    };
    let token = init_user(&user, config).await.unwrap();

    let f = PostFournisseurRequest {
        code: "f-01".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
    
    };

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0)
        .json(&f)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();

    assert_eq!(res.email, "sgb@gmail.com");
    assert_eq!(res.sigle, f.sigle);
}



pub async fn post_fournisseur_without_suitable_role(config: &Config) {

    init_db(config).unwrap();

    
    let user = PostUserRequest {

        email: "test@email.com".to_string(),
        password: "password".to_string(),
        roles: "CE,DAFP".to_string(),
    };
    let token = init_user(&user, config).await.unwrap();

    let f = PostFournisseurRequest {
        code: "f-16".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
    
    };

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0)
        .json(&f)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert_eq!(res, "No permission to change underlying resource");

}


pub async fn list_fournisseurs(config: &Config){

    init_db(config).unwrap();

    let user = PostUserRequest {

        email: "test@email.com".to_string(),
        password: "password".to_string(),
        roles: "ADMIN,CE,DAFP".to_string(),
    };
    let token = init_user(&user, config).await.unwrap();

    let f1 = PostFournisseurRequest {

        code: "f-01".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
    
    };

    let f2 = PostFournisseurRequest{
 
        code: "f-02".to_string(),
        sigle: "GHI".to_string(),
        designation: "societe du sud".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "ghi@gmail.com".to_string(),
       
    };

    let f3 = PostFournisseurRequest {

        code: "f-03".to_string(),
        sigle: "GTI".to_string(),
        designation: "societe du sud".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "gti@gmail.com".to_string(),
    
    };

    let client = reqwest::Client::new();

    client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f1)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();

    client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f2)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();


    client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f3)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();


    let res = client
        .get("http://localhost:3030/fournisseurs?limit=2&offset=0")
        .header("Authorization", token.0.clone())
        .send()
        .await
        .unwrap()
        .json::<Vec<GetFournisseurAnswer>>()
        .await
        .unwrap();

    assert_eq!(res.len(),2);

    let res = client
        .get("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .send()
        .await
        .unwrap()
        .json::<Vec<GetFournisseurAnswer>>()
        .await
        .unwrap();

    assert_eq!(res.len(),4); //4 because of the previous add_fournisseur test
}



pub async fn get_fournisseur_by_id(config: &Config){

    init_db(config).unwrap();

    let user = PostUserRequest {

        email: "test@email.com".to_string(),
        password: "password".to_string(),
        roles: "ADMIN,CE,DAFP".to_string(),
    };
    let token = init_user(&user, config).await.unwrap();

    let f1 = PostFournisseurRequest {
        code: "f-05".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
       
    };

    let client = reqwest::Client::new();

    let post_res = client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f1)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();

    let raw_url = "http://localhost:3030/fournisseurs/".to_owned();
    let id = post_res.id.0.clone();
    let get_url = format!("{raw_url}{id}");

    let get_res = client
        .get(get_url)
        .header("Authorization", token.0.clone())
        .send()
        .await
        .unwrap()
        .json::<GetFournisseurAnswer>()
        .await
        .unwrap();

    assert_eq!(get_res.id,post_res.id.0);


}
 
pub async fn get_fournisseur_without_auth_token(config: &Config){

    init_db(config).unwrap();

    let user = PostUserRequest {
        email: "test@email.com".to_string(),
        password: "password".to_string(),
        roles: "ADMIN,CE,DAFP".to_string(),
    };
    let token = init_user(&user, config).await.unwrap();

    let f1 = PostFournisseurRequest {
        code: "f-15".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
       
    };

    let client = reqwest::Client::new();

    let post_res = client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f1)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();

    let raw_url = "http://localhost:3030/fournisseurs/".to_owned();
    let id = post_res.id.0.clone();
    let get_url = format!("{raw_url}{id}");

    let get_res = client
        .get(get_url)
        //.header("Authorization", token.0.clone())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert_eq!(get_res,"Route not found");
}

pub async fn get_fournisseur_with_wrong_id(config: &Config){

    init_db(config).unwrap();

    let user = PostUserRequest {

        email: "test@email.com".to_string(),
        password: "password".to_string(),
        roles: "ADMIN,CE,DAFP".to_string(),
    };
    let token = init_user(&user, config).await.unwrap();

    let f1 = PostFournisseurRequest {
        code: "f-05".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
       
    };

    let client = reqwest::Client::new();

    client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f1)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();

    let raw_url = "http://localhost:3030/fournisseurs/".to_owned();
    let id = "aaa-bbb";
    let get_url = format!("{raw_url}{id}");

    let get_res = client
        .get(get_url)
        .header("Authorization", token.0.clone())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

        
    assert_eq!(get_res,"Entity not found");


}


pub async fn put_fournisseur(config: &Config){

    init_db(config).unwrap();

    let user = PostUserRequest {
        email: "test@email.com".to_string(),
        password: "password".to_string(),
        roles: "ADMIN,CE,DAFP".to_string(),
    };
    let token = init_user(&user, config).await.unwrap();

    let f1 = PostFournisseurRequest {

        code: "f-01".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
   
    };

    let client = reqwest::Client::new();
    let post_res = client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f1)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();

    
    let f2 = PutFournisseurRequest {
            code: "f-02".to_string(),
            sigle: "ASB".to_string(),
            designation: "Association des societés".to_string(),
            telephone: "07-08-08-08-08".to_string(),
            email: "asb@gmail.com".to_string(),
    
    };

    let raw_url = "http://localhost:3030/fournisseurs/".to_owned();
    let id = post_res.id.0;
    let put_url = format!("{raw_url}{id}");

    let put_res = client
        .put(put_url)
        .header("Authorization", token.0)
        .json(&f2)
        .send()
        .await
        .unwrap()
        .json::<PutFournisseurAnswer>()
        .await
        .unwrap();

    assert_eq!(put_res.code,"f-02");
    assert_eq!(put_res.sigle,"ASB");
    assert_eq!(put_res.designation,"Association des societés");
    assert_eq!(put_res.telephone,"07-08-08-08-08");
    assert_eq!(put_res.email,"asb@gmail.com");



}


pub async fn delete_fournisseur(config: &Config){

    init_db(config);

    let user = PostUserRequest {

        email: "test@email.com".to_string(),
        password: "password".to_string(),
        roles: "ADMIN,CE,DAFP".to_string(),
    };
    let token = init_user(&user, config).await.unwrap();

    let f1 = PostFournisseurRequest {
        code: "f-09".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
   
    };

    let client = reqwest::Client::new();
    let post_res = client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f1)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();

    let raw_url = "http://localhost:3030/fournisseurs/".to_owned();
    let id = post_res.id.0;
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

pub async fn get_fournisseur_dossiers(config: &Config){

    init_db(config);
    let user = PostUserRequest {

        email: "test@email.com".to_string(),
        password: "password".to_string(),
        roles: "ADMIN,CE,DAFP".to_string(),
    };
    let token = init_user(&user, config).await.unwrap();

    let f1 = PostFournisseurRequest {
        code: "f-20".to_string(),
        sigle: "SGB".to_string(),
        designation: "societe de societé".to_string(),
        telephone: "07-07-08-08-08".to_string(),
        email: "sgb@gmail.com".to_string(),
   
    };

    let client = reqwest::Client::new();
    let post_res = client
        .post("http://localhost:3030/fournisseurs")
        .header("Authorization", token.0.clone())
        .json(&f1)
        .send()
        .await
        .unwrap()
        .json::<PostFournisseurAnswer>()
        .await
        .unwrap();

   
}