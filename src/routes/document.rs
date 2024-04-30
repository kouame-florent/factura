
use tracing::{event, instrument, Level};


use crate::{
    store::document::DocumentStore, types::{
         document::{DocumentId, NewDocument, UpdatedDocument}, fichier::{Fichier, NewFichier}, pagination::{extract_pagination, 
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

