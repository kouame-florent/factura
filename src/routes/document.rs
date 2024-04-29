use std::task;

use bytes::BufMut;
use futures_util::{StreamExt, TryStreamExt};
use warp::filters::multipart::Part;
use warp::multipart::FormData;
use warp::Filter;
use tracing::{event, instrument, Level};


use crate::{
    store::document::DocumentStore, types::{
         document::{NewDocument, UpdatedDocument}, pagination::{extract_pagination, 
        Pagination} 
    }
    
    
};

use crate::types::account::{Session,Roles};
use crate::store::authentication::AuthStore;



#[instrument]
pub async fn add_document(
    session: Session,
    store: DocumentStore,
    auth_store: AuthStore,
    document: NewDocument,
) -> Result<impl warp::Reply, warp::Rejection>{
    let account_id = session.account_id;
    let email = match auth_store.get_email(account_id.0.clone()).await {  
        Ok(email) => {
            email
        },
        Err(_) => {
            return Err(warp::reject::custom(handle_errors::Error::Unauthorized));
        }
    };
    let admin: Roles = Roles::ADMIN;
    if auth_store.has_authorization(account_id.0, admin.as_str().to_string()).await?{
        match store.add_document(document, email.to_string()).await {
            Ok(f) => {
                Ok(warp::reply::json(&f))
            }
            Err(e) => Err(warp::reject::custom(e)),
        }

    }else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
    
}


// #[instrument]
// pub async fn add_document(
//     form: warp::multipart::FormData,
// ) -> Result<impl warp::Reply, warp::Rejection>{

//     let field_names: Vec<_> = form.and_then(|mut field| async move{
//         let mut bytes: Vec<u8> = Vec::new();

//         // field.data() only returns a piece of the content, you should call over it until it replies None
//         while let Some(content) = field.data().await {
//             let content = content.unwrap();
//             bytes.put(content);
//         }
//         Ok((
//             field.name().to_string(),
//            // field.filename().unwrap().to_string(),
//             String::from_utf8_lossy(&*bytes).to_string(),
            
//         ))
//     })
//     .try_collect()
//     .await
//     .unwrap();


    
//     //println!("Form parts {:?}", form);
    
//     Ok::<_, warp::Rejection>(format!("{:?}", field_names))
// }


// #[instrument]
// pub async fn get_document(

// ) -> Result<impl warp::Reply, warp::Rejection>{

//     Ok(warp::reply::json(&"get forunisseur"))
// }