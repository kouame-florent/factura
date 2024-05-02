use std::process::Command;
use std::io::{self, Write};
use futures_util::future::FutureExt; 

use factura::{config, handle_errors, oneshot, setup_db_connection};
use dtos::user::PostUserRequest; 


use handlers::test_init::{
    init_db,
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



mod dtos;
mod handlers;


#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {

    dotenv::dotenv().ok();
    let config = config::Config::new().expect("Config can't be set");

    // let s = Command::new("sqlx")
    //     .arg("database")
    //     .arg("drop")
    //     .arg("--database-url")
    //     .arg(format!("postgres://{}:{}@{}:{}/{}",
    //             config.db_user,
    //             config.db_password,
    //             config.db_host,
    //             config.db_port,
    //             config.db_name
    //     )).arg("-y")
    //     .output()
    //     .expect("sqlx command failed to start");

    // io::stdout().write_all(&s.stderr).unwrap();

    // let s = Command::new("sqlx")
    //     .arg("database")
    //     .arg("create")
    //     .arg("--database-url")
    //     .arg(format!("postgres://{}:{}@{}:{}/{}",
    //             config.db_user,
    //             config.db_password,
    //             config.db_host,
    //             config.db_port,
    //             config.db_name
    //     ))
    //     .output()
    //     .expect("sqlx command failed to start");

    // // Exdcute DB commands to drop and create a new test database
    // io::stdout().write_all(&s.stderr).unwrap();

    
    init_db(&config).unwrap();

     // set up a new store instance with a db connection pool
    let conn = setup_db_connection(&config).await?;

    // start the server and listen for a sender signal to shut it down
    let handler = oneshot(&config,true, conn).await;

    let u = PostUserRequest {
        email: "test@email.com".to_string(),
        password: "password".to_string(),
        roles: "ADMIN,CE,DAFP".to_string(),
    };
        
   // let token;

    print!("Running register_new_user...");
    let result = std::panic::AssertUnwindSafe(register_new_user(&u, &config)).catch_unwind().await;
    match result {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    let u1 = PostUserRequest {
        email: "test1@email.com".to_string(),
        password: "password".to_string(),
        roles: "ADMIN,CE,DAFP".to_string(),
    };
        

    print!("Running login...");
    match std::panic::AssertUnwindSafe(login(&u1, &config)).catch_unwind().await {
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
    match std::panic::AssertUnwindSafe(post_fournisseur(&config)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    // let fu = User {
    //     id: "aa-xx".to_string(),
    //     email: "xx@email.com".to_string(),
    //     password: "password".to_string(),
    //     roles: "CE".to_string(),
    // };

    // let w_token = get_token_for(fu).await.unwrap();

    print!("Running post_fournisseur_without_suitable_role...");
    match std::panic::AssertUnwindSafe(post_fournisseur_without_suitable_role(&config)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running get_fournisseurs...");
    match std::panic::AssertUnwindSafe(list_fournisseurs(&config)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running get_fournisseur_by_id...");
    match std::panic::AssertUnwindSafe(get_fournisseur_by_id(&config)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running get_fournisseur_with_wrong_id...");
    match std::panic::AssertUnwindSafe(get_fournisseur_with_wrong_id(&config)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running get_fournisseur_without_auth_token...");
    match std::panic::AssertUnwindSafe(get_fournisseur_without_auth_token(&config)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running put_fournisseur...");
    match std::panic::AssertUnwindSafe(put_fournisseur(&config)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running delete_fournisseur...");
    match std::panic::AssertUnwindSafe(delete_fournisseur(&config)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    //test dossier fournisseur

    print!("Running post_dossier_fournisseur...");
    match std::panic::AssertUnwindSafe(post_dossier_fournisseur(&config)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running put_dossier_fournisseur...");
    match std::panic::AssertUnwindSafe(put_dossier_fournisseur(&config)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }


    print!("Running get_dossier_fournisseur_by_id...");
    match std::panic::AssertUnwindSafe(get_dossier_fournisseur_by_id(&config)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running list_dossiers_fournisseurs...");
    match std::panic::AssertUnwindSafe(list_dossiers_fournisseurs(&config)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running post_document...");
    match std::panic::AssertUnwindSafe(post_document(&config)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running get_document...");
    match std::panic::AssertUnwindSafe(get_document_by_id(&config)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }



    let _ = handler.sender.send(1);

    Ok(())
}
