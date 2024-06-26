use warp::{http::Method, reply::Reply, Filter};
use tracing_subscriber::fmt::format::FmtSpan;
pub use handle_errors;
use tokio::sync::{oneshot, oneshot::Sender};

pub mod types; 
mod routes;
mod store;
pub mod config;

pub struct OneshotHandler {
    pub sender: Sender<i32>,
}

async fn build_routes(conn: store::db_connection::DBConnection) -> impl Filter<Extract = impl Reply> + Clone{

    let auth_store = store::authentication::AuthStore::new(conn.pool.clone()).await;
    let fournisseur_store = store::fournissueur::FournisseurStore::new(conn.pool.clone()).await;
    let dossier_fournisseur_store = store::dossier_fournisseur::DossierFournisseurStore::new(conn.pool.clone()).await;
    let document_store = store::document::DocumentStore::new(conn.pool.clone()).await;
    let fichier_store = store::fichier::FichierStore::new(conn.pool.clone()).await;


    let auth_store_filter = warp::any().map(move || auth_store.clone());
    let fournisseur_store_filter = warp::any().map(move || fournisseur_store.clone() );
    let dossier_fournisseur_store_filter = warp::any().map(move || dossier_fournisseur_store.clone() );
    let document_store_filter = warp::any().map(move || document_store.clone());
    let fichier_store_filter = warp::any().map(move || fichier_store.clone());
 

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);


    let add_fournisseur = warp::post() 
        .and(warp::path("fournisseurs"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(fournisseur_store_filter.clone())
        .and(auth_store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::fournisseur::add_fournisseur);

    let get_fournisseurs = warp::get()
        .and(warp::path("fournisseurs"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(warp::query())
        .and(fournisseur_store_filter.clone())
        .and(auth_store_filter.clone())
        .and_then(routes::fournisseur::get_fournisseurs);

   
    let update_fournisseur = warp::put()
        .and(warp::path!("fournisseurs" / String))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(fournisseur_store_filter.clone())
        .and(auth_store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::fournisseur::update_fournisseur);


    let get_fournisseur = warp::get()
        .and(warp::path!("fournisseurs" / String))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(fournisseur_store_filter.clone())
        .and(auth_store_filter.clone())
        .and_then(routes::fournisseur::get_fournisseur);

    let delete_fournisseur = warp::delete()
        .and(warp::path!("fournisseurs" / String))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(fournisseur_store_filter.clone())
        .and(auth_store_filter.clone())
        .and_then(routes::fournisseur::delete_fournisseur);

    let get_dossiers_by_fournisseur_id = warp::get()
        .and(warp::path!("fournisseurs" / String / "dossiers"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(fournisseur_store_filter.clone())
        .and(auth_store_filter.clone())
        .and_then(routes::fournisseur::get_dossiers);

    let get_specific_dossier_by_fournisseur_id = warp::get()
        .and(warp::path!("fournisseurs" / String / "dossiers" / String))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(fournisseur_store_filter.clone())
        .and(auth_store_filter.clone())
        .and_then(routes::fournisseur::get_dossier);


    //dossier fournisseur resources

    let add_dossier_fournisseur = warp::post() 
        .and(warp::path("dossiers-fournisseurs"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(dossier_fournisseur_store_filter.clone())
        .and(auth_store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::dossier_fournisseur::add_dossier_fournisseur);


    let update_dossier_fournisseur = warp::put()
        .and(warp::path!("dossiers-fournisseurs" / String))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(dossier_fournisseur_store_filter.clone())
        .and(auth_store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::dossier_fournisseur::update_dossier_fournisseur);


    let get_dossier_fournisseur = warp::get()
        .and(warp::path!("dossiers-fournisseurs" / String))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(dossier_fournisseur_store_filter.clone())
        .and(auth_store_filter.clone())
        .and_then(routes::dossier_fournisseur::get_dossier_fournisseur);

    let get_dossiers_fournisseurs = warp::get()
        .and(warp::path!("dossiers-fournisseurs"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(warp::query())
        .and(dossier_fournisseur_store_filter.clone())
        .and(auth_store_filter.clone())
        .and_then(routes::dossier_fournisseur::get_dossiers_fournisseurs);


    let delete_dossier_fournisseur = warp::delete()
        .and(warp::path!("dossiers-fournisseurs" / String))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(dossier_fournisseur_store_filter.clone())
        .and(auth_store_filter.clone())
        .and_then(routes::dossier_fournisseur::delete_dossier_fournisseur);

    let add_document = warp::post() 
        .and(warp::path("documents"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(document_store_filter.clone())
        .and(auth_store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::document::add_document);


    let get_document = warp::get()
        .and(warp::path!("documents" / String))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(document_store_filter.clone())
        .and(auth_store_filter.clone())
        .and_then(routes::document::get_document);

    let get_documents = warp::get()
        .and(warp::path!("documents"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(warp::query())
        .and(document_store_filter.clone())
        .and(auth_store_filter.clone())
        .and_then(routes::document::get_documents);

    let update_document =  warp::put()
        .and(warp::path!("documents" / String))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(document_store_filter.clone())
        .and(auth_store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::document::update_document);

    let delete_document = warp::delete()
        .and(warp::path!("documents" / String))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(document_store_filter.clone())
        .and(auth_store_filter.clone())
        .and_then(routes::document::delete_document);


    let add_fichier = warp::multipart::form()
        .and(warp::path!("fichiers"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(fichier_store_filter.clone())
        .and(auth_store_filter.clone())
        .and_then(routes::fichier::add_fichier);

    // let get_fichier = warp::get()
    //     .and(warp::path!("fichiers" / String))
    //     .and(warp::path::end())
    //     .and(warp::fs::dir("/home/florent/backup_23_02_2024/project-anrmp/uploads"))
    //     .and(routes::authentication::auth())
    //     .and(fichier_store_filter.clone())
    //     .and(auth_store_filter.clone())
    //     .and_then(routes::fichier::get_fichier);

    let get_fichier = warp::get()
        .and(warp::path!("fichiers" / String))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(fichier_store_filter.clone())
        .and(auth_store_filter.clone())
        .and_then(routes::fichier::get_fichier);
        

    let registration = warp::post() 
        .and(warp::path("registrations"))
        .and(warp::path::end())
        .and(auth_store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(auth_store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::login);

    //document 

    

    // let add_document = warp::multipart::form()
    //     .and_then(routes::document::add_document);


    get_fournisseurs
        .or(update_fournisseur)
        .or(add_fournisseur)
        .or(get_fournisseur)
        .or(delete_fournisseur)
        .or(get_dossiers_by_fournisseur_id)
        .or(get_specific_dossier_by_fournisseur_id)
        .or(add_dossier_fournisseur)
        .or(get_dossier_fournisseur)
        .or(get_dossiers_fournisseurs)
        .or(update_dossier_fournisseur)
        .or(delete_dossier_fournisseur)
        .or(add_document)
        .or(get_document)
        .or(get_documents)
        .or(update_document)
        .or(delete_document)
        .or(add_fichier)
        .or(get_fichier)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace::request())
        .recover(handle_errors::return_error)



}

pub async fn setup_db_connection(config: &config::Config) -> Result<store::db_connection::DBConnection, handle_errors::Error> {
   
    let db_url = &format!("postgres://{}:{}@{}:{}/{}",
        config.db_user,
        config.db_password,
        config.db_host,
        config.db_port,
        config.db_name
    );

    let conn = store::db_connection::DBConnection::new(db_url).await;

    sqlx::migrate!()
        .run(&conn.pool.clone())
        .await
        .expect("Cannot run migration !");

    Ok(conn)
}

pub async fn run(config: &config::Config, conn: store::db_connection::DBConnection) {

    let log_filter = format!(
        "handle_errors={},factura={},sqlx={},warp={}",
        config.log_level, config.log_level, config.log_level,config.log_level
    );

    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes.
        // This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let routes = build_routes(conn).await;
    warp::serve(routes).run(([127, 0, 0, 1], config.app_port )).await;
}


pub async fn oneshot(config: &config::Config, show_traces: bool, conn: store::db_connection::DBConnection) -> OneshotHandler {
    
    let routes = build_routes(conn).await;
    let (tx, rx) = oneshot::channel::<i32>();

    let socket: std::net::SocketAddr = "127.0.0.1:3030"
        .to_string()
        .parse()
        .expect("Not a valid address");

    if show_traces {
        let log_filter = format!(
            "handle_errors={},factura={},sqlx={},warp={}",
            config.log_level, config.log_level, config.log_level,config.log_level
        );
        
        tracing_subscriber::fmt()
            // Use the filter we built above to determine which traces to record.
            .with_env_filter(log_filter)
            // Record an event when each span closes.
            // This can be used to time our
            // routes' durations!
            .with_span_events(FmtSpan::CLOSE)
            .init();

    }
    
    let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(socket, async {
        rx.await.ok();
    });

    tokio::task::spawn(server);

    OneshotHandler { sender: tx }
}