use std::task;

use bytes::BufMut;
use futures_util::{StreamExt, TryStreamExt};
use uuid::Uuid;
use warp::filters::multipart::Part;
use warp::multipart::FormData;
use warp::Filter;
use tracing::{event, instrument, Level};

use crate::types::{document::DocumentId, fichier::Fichier};


use crate::types::account::{Session,Roles};
use crate::store::authentication::AuthStore;



#[instrument]
pub async fn upload(
    form: warp::multipart::FormData,
    session: Session,
    auth_store: AuthStore,
) -> Result<impl warp::Reply, warp::Rejection>{

    let mut fichier = Fichier{
        document_id: DocumentId("".to_string()),
        file_name: "".to_string(),
        file_size: 0i64,
        mime_type: "".to_string(),
        file_reference: "".to_string(),
    };

    // let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
    //     eprintln!("form error: {}", e);
    //     warp::reject::reject()
    // })?;

    let mut parts = form.into_stream();

    while let Some(part) = parts.next().await{
        match part{
            Ok(p) => {
               if p.name() == "file" {

                    let content_type = p.content_type();
                    
                    match content_type.clone() {
                        Some(t) => fichier.mime_type = t.to_string(),
                        None => fichier.mime_type = "text/plain".to_string()
                    };

                    let file_ext = match get_file_extension(content_type)
                            .await {
                                Ok(ext) => ext,
                                Err(e) => {
                                    return Err(e);
                                }

                    };
                    
                    let content = match get_part_data(p)
                        .await {
                            Ok(content) => content,
                            Err(e) =>  {
                                return Err::<Vec<u8>,warp::Rejection>(e);
                            }
                    };    

                    save_to_disk(file_ext, content).await.unwrap();
                
                }else if p.name() == "document_id"{
                    let doc_id = match get_part_data(p)
                        .await {
                            Ok(content) => String::from_utf8(content) 
                                                        .unwrap_or("not a valid document_id".to_string()) ,
                            Err(_) =>  "not a valid document_id".to_string()
                    };

                    fichier.document_id = DocumentId(doc_id.to_string());

                }
            },
            
            Err(_) => return Err(warp::reject::reject())

        }


    }

    
    // for p in parts {
    //     if p.name() == "file" {

    //         let content_type = p.content_type();
            
    //         match content_type.clone() {
    //             Some(t) => fichier.mime_type = t.to_string(),
    //             None => fichier.mime_type = "text/plain".to_string()
    //         };

    //         let file_ext = match get_file_extension(content_type)
    //                 .await {
    //                     Ok(ext) => ext,
    //                     Err(e) => {
    //                         return Err(e);
    //                     }

    //         };
            
    //         let content = match get_part_data(p)
    //             .await {
    //                 Ok(content) => content,
    //                 Err(e) =>  {
    //                     return Err::<Vec<u8>,warp::Rejection>(e);
    //                 }
    //         };    

    //        save_to_disk(file_ext, content).await.unwrap();
           
    //     }else if p.name() == "document_id"{
    //         let doc_id = match get_part_data(p)
    //             .await {
    //                 Ok(content) => String::from_utf8(content) 
    //                                             .unwrap_or("not a valid document_id".to_string()) ,
    //                 Err(_) =>  "not a valid document_id".to_string()
    //         };

    //         fichier.document_id = DocumentId(doc_id.to_string());

    //     }
    // } 

    Err(warp::reject::custom(handle_errors::Error::Unauthorized))

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
            warp::reject::reject()
        })
}


async fn save_to_disk(file_ext: String, content: Vec<u8>) -> Result<(), warp::Rejection>{
    let file_name = format!("/home/florent/backup_23_02_2024/project-anrmp/uploads/{}.{}", Uuid::new_v4().to_string(), file_ext);
    tokio::fs::write(&file_name, content)
            .await.map_err(|e| {
                eprint!("error writing file: {}", e);
                warp::reject::reject()
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
            v => {
                eprintln!("invalid file type found: {}", v);
                return Err(warp::reject::reject());
            }
        },
        None => {
            eprintln!("file type could not be determined");
            return Err(warp::reject::reject());
        }

    }

    Ok(file_ending.to_string())

}