use std::str::FromStr;
use bytes::BufMut;
use futures_util::{StreamExt, TryStreamExt};
use uuid::Uuid;
use warp::filters::fs::File;
use warp::filters::multipart::Part;
use tracing::{event, instrument, Level};
//use warp::reply::Response;
//use warp::Filter;
use crate::types::{document::DocumentId, fichier::NewFichier};
use crate::types::account::{Session,Roles};
use crate::store::authentication::AuthStore;
use crate::store::fichier::FichierStore;
use warp::http::header::{HeaderMap, HeaderValue};
use warp::{Filter, http::Response};



#[instrument]
pub async fn add_fichier(
    form: warp::multipart::FormData,
    session: Session,
    store: FichierStore,
    auth_store: AuthStore,
) -> Result<impl warp::Reply, warp::Rejection>{

    let account_id = session.account_id;
    let email = auth_store.get_email(account_id.0.clone()).await?;
    let admin: Roles = Roles::ADMIN;

    if auth_store.has_authorization(account_id.0, admin.as_str().to_string()).await?{

        let fichier = handle_multiparts(form).await?;
        match store.add_fichier(fichier, email.to_string()).await{
            Ok(f) => {
                Ok(warp::reply::json(&f))
            },
            Err(e) => Err(warp::reject::custom(e))
        }
    

    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
    
    //Ok(warp::reply::json(&"file saved successfully"))

}

#[instrument]
pub async fn get_fichier(
    id: String,
    session: Session,
    store: FichierStore,
    auth_store: AuthStore,
) -> Result<impl warp::Reply, warp::Rejection>{

    let account_id = session.account_id;
    //let email = auth_store.get_email(account_id.0.clone()).await?;
    let admin: Roles = Roles::CE;

    if auth_store.has_authorization(account_id.0, admin.as_str().to_string()).await?{
        match store.get_fichier(id).await {
            Ok(f) => {
                let file_name = format!("/home/florent/backup_23_02_2024/project-anrmp/uploads/{}.{}", f.data_pointer, "png");
                println!("---File path: {:?}",file_name.clone());
                let content = tokio::fs::read(file_name).await;
                match content {
                    Ok(c) => {
                       // Ok(warp::reply::Response::new(c.into()))
                        //let mut headers = HeaderMap::new();
                        //headers.insert("Content-Type", HeaderValue::from_static(f.mime_type.as_str()));
                        //warp::reply::with::headers(headers)::wi
                        Ok(Response::builder()
                            .header("Content-Type", f.mime_type.as_str())
                            .body(c).unwrap()
                        )
                    },
                    Err(_) => Err(warp::reject::custom(handle_errors::Error::FileNotFound))
                }
            },
            Err(e) => Err(warp::reject::custom(e)),
        }
    

    }else{
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
    
}

// async fn download(){

// }

async fn handle_multiparts(form: warp::multipart::FormData) -> Result<NewFichier, warp::Rejection>{

    let mut new_fichier = NewFichier{
        document_id: DocumentId("".to_string()),
        file_name: "".to_string(),
        file_size: 0i64,
        mime_type: "".to_string(),
        data_pointer: "".to_string(),
    };

    let mut parts = form.into_stream();

    while let Some(part) = parts.next().await{

        match part {
            Ok(p) => {
                if p.name() == "file" {
                    let content_type = p.content_type();
                    let mime = content_type.unwrap_or("text/plain");

                    new_fichier.mime_type = mime.to_string();
                    new_fichier.file_name = String::from_str(p.filename().unwrap()).unwrap() ;

                    let file_ext = get_file_extension(content_type).await?;
                    let content = get_part_data(p).await?;
                                       
                    let d_pointer =  Uuid::new_v4().to_string();
                    new_fichier.file_size = content.len() as i64;  
                    new_fichier.data_pointer = d_pointer.clone();

                    save_to_disk(file_ext, d_pointer, content).await.unwrap();

                }else {
                    let doc_id = match get_part_data(p)
                       .await {
                            Ok(content) => String::from_utf8(content) 
                                                         .unwrap_or("not_a_valid_document_id".to_string()) ,
                             Err(_) => return Err(warp::reject::custom(handle_errors::Error::InvalidDocumentId))
                     };

                     new_fichier.document_id = DocumentId(doc_id.to_string());

                }
            },
            Err(_) => return Err(warp::reject::custom(handle_errors::Error::MultipartParsingError))
       }
            
        
    }

    Ok(new_fichier)

}


async fn get_part_data(part: Part) -> Result<Vec<u8>, warp::Rejection>{
    part.stream()
        .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
        })
        .await
        .map_err(|e| {
            eprintln!("reading file error: {}", e);
            warp::reject::custom(handle_errors::Error::MuiltipartFileReadingError)
        })
}

async fn get_file_extension(content_type: Option<&str>) -> Result<String, warp::Rejection>{

    let file_ending;
    match content_type {
        Some(file_type) => match file_type {
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
                return Err(warp::reject::custom(handle_errors::Error::UnsupportedFileType));
            }
        },
        None => {
            eprintln!("file type could not be determined");
            return Err(warp::reject::custom(handle_errors::Error::UnsupportedFileType));
        }

    }

    Ok(file_ending.to_string())

}


async fn save_to_disk(file_ext: String,data_pointer: String, content: Vec<u8>) -> Result<(), warp::Rejection>{
    let file_name = format!("/home/florent/backup_23_02_2024/project-anrmp/uploads/{}.{}", data_pointer, file_ext);
    tokio::fs::write(&file_name, content)
            .await.map_err(|e| {
                eprint!("error writing file: {}", e);
                warp::reject::reject()
            })
}