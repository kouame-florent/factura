use std::process::Command;
use std::io::{self, Write};
use futures_util::future::FutureExt; 

use factura::{config, handle_errors, oneshot, setup_db_connection};
use dtos::user::PostUserRequest; 


use handlers::utils::{
   create_db,
   drop_db,
};

use handlers::fournisseur::{
    post_fournisseur,
    post_fournisseur_without_suitable_role,
    put_fournisseur,
    get_fournisseur_by_id,
    list_fournisseurs,
    get_fournisseur_with_wrong_id,
    get_fournisseur_without_auth_token,
    delete_fournisseur,
};

use handlers::dossier_fournisseur::{ 
    post_dossier_fournisseur,
    put_dossier_fournisseur,
    get_dossier_fournisseur_by_id,
    list_dossiers_fournisseurs
};

use handlers::document::{
    post_document,
    get_document_by_id,
};

use handlers::user::{
    login,
    register_new_user,
    get_token_for,
};

use crate::handlers::document::{delete_document, list_documents, update_document};
use crate::handlers::utils::get_email;


mod dtos;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {

    dotenv::dotenv().ok();
    let config = config::Config::new().expect("Config can't be set");

    
   // drop_db(&config).await.unwrap();
   // create_db(&config).await.unwrap();

     // set up a new store instance with a db connection pool
    let conn = setup_db_connection(&config).await?;

    // start the server and listen for a sender signal to shut it down
    let handler = oneshot(&config,true, conn).await;


    print!("Running register_new_user...");
    let result = std::panic::AssertUnwindSafe(register_new_user()).catch_unwind().await;
    match result {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

        

    print!("Running login...");
    match std::panic::AssertUnwindSafe(login()).catch_unwind().await {
        Ok(_) => {
            //token = t;
            println!("✓");
        },
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }


    print!("Running post_fournisseur...");
    match std::panic::AssertUnwindSafe(post_fournisseur()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }


    print!("Running post_fournisseur_without_suitable_role...");
    match std::panic::AssertUnwindSafe(post_fournisseur_without_suitable_role()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running list_fournisseurs...");
    match std::panic::AssertUnwindSafe(list_fournisseurs()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running get_fournisseur_by_id...");
    match std::panic::AssertUnwindSafe(get_fournisseur_by_id()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running get_fournisseur_with_wrong_id...");
    match std::panic::AssertUnwindSafe(get_fournisseur_with_wrong_id()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running get_fournisseur_without_auth_token...");
    match std::panic::AssertUnwindSafe(get_fournisseur_without_auth_token()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running put_fournisseur...");
    match std::panic::AssertUnwindSafe(put_fournisseur()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running delete_fournisseur...");
    match std::panic::AssertUnwindSafe(delete_fournisseur()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    //test dossier fournisseur

    print!("Running post_dossier_fournisseur...");
    match std::panic::AssertUnwindSafe(post_dossier_fournisseur()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running put_dossier_fournisseur...");
    match std::panic::AssertUnwindSafe(put_dossier_fournisseur()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }


    print!("Running get_dossier_fournisseur_by_id...");
    match std::panic::AssertUnwindSafe(get_dossier_fournisseur_by_id()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running list_dossiers_fournisseurs...");
    match std::panic::AssertUnwindSafe(list_dossiers_fournisseurs()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running post_document...");
    match std::panic::AssertUnwindSafe(post_document()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running put_documents...");
    match std::panic::AssertUnwindSafe(update_document()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running get_document...");
    match std::panic::AssertUnwindSafe(get_document_by_id()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running list_documents...");
    match std::panic::AssertUnwindSafe(list_documents()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running delete_document...");
    match std::panic::AssertUnwindSafe(delete_document()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }


    let _ = handler.sender.send(1);

    Ok(())
}
