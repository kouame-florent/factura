
use routes::fournisseur::get_dossiers;
use warp::{http::Method, Filter};
use tracing_subscriber::fmt::format::FmtSpan;
use handle_errors::return_error;

mod types; 
mod routes;
mod store;



#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {

    dotenv::dotenv().ok();

    let log_filter = std::env::var("LOG_LEVEL") 
        .unwrap_or_else(|_| "error=warn,factura=info,warp=error".to_owned());

    let app_port = std::env::var("APPLICATION_PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8080))
        .map_err(|e| handle_errors::Error::ParseError(e))?;


    let db_port = std::env::var("DATABASE_PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(5432))
        .map_err(|e| handle_errors::Error::ParseError(e))?;

    let db_user =  std::env::var("DATABASE_USER")
        .map_err(|e| handle_errors::Error::ValueNotSet(e))?;

    let db_password =  std::env::var("DATABASE_PASSWORD")
        .map_err(|e| handle_errors::Error::ValueNotSet(e))?;

    let db_host =  std::env::var("DATABASE_HOST")
        .map_err(|e| handle_errors::Error::ValueNotSet(e))?;

    let db_name =  std::env::var("DATABASE_NAME")
        .map_err(|e| handle_errors::Error::ValueNotSet(e))?;
        

    let db_url = &format!("postgres://{}:{}@{}:{}/{}",db_user,
        db_password,
        db_host,
        db_port,
        db_name
    );
        

    let conn = store::db_connection::DBConnection::new(db_url).await;

    sqlx::migrate!()
        .run(&conn.pool.clone())
        .await
        .expect("Cannot run migration !");
        

    let auth_store = store::authentication::AuthStore::new(conn.pool.clone()).await;
    let fournisseur_store = store::fournissueur::FournisseurStore::new(conn.pool.clone()).await;
    let dossier_fournisseur_store = store::dossier_fournisseur::DossierFournisseurStore::new(conn.pool.clone()).await;

    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes.
        // This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    

    let auth_store_filter = warp::any().map(move || auth_store.clone());
    let fournisseur_store_filter = warp::any().map(move || fournisseur_store.clone() );
    let dossier_fournisseur_store_filter = warp::any().map(move || dossier_fournisseur_store.clone() );
    // let log = warp::log::custom(|info| {
    //     eprintln!(
    //         "{} {} {}",
    //         info.method(),
    //         info.path(),
    //         info.status(),
            
    //     );
    // });

    let add_fournisseur = warp::post() 
        .and(warp::path("fournisseurs"))
        .and(warp::path::end())
        .and(fournisseur_store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::fournisseur::add_fournisseur);

    let get_fournisseurs = warp::get()
        .and(warp::path("fournisseurs"))
        .and(warp::path::end())
        .and(warp::query())
        .and(fournisseur_store_filter.clone())
        .and_then(routes::fournisseur::get_fournisseurs);

   
    let update_fournisseur = warp::put()
        .and(warp::path!("fournisseurs" / String))
        //.and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(fournisseur_store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::fournisseur::update_fournisseur);

    let get_fournisseur = warp::get()
        .and(warp::path!("fournisseurs" / String))
        .and(warp::path::end())
        .and(fournisseur_store_filter.clone())
        .and_then(routes::fournisseur::get_fournisseur);

    let delete_fournisseur = warp::delete()
        .and(warp::path!("fournisseurs" / String))
        .and(warp::path::end())
        .and(fournisseur_store_filter.clone())
        .and_then(routes::fournisseur::delete_fournisseur);

    let get_dossiers_by_fournisseur_id = warp::get()
        .and(warp::path!("fournisseurs" / String / "dossiers"))
        .and(warp::path::end())
        .and(fournisseur_store_filter.clone())
        .and_then(routes::fournisseur::get_dossiers);

    let get_specific_dossier_by_fournisseur_id = warp::get()
        .and(warp::path!("fournisseurs" / String / "dossiers" / String))
        .and(warp::path::end())
        .and(fournisseur_store_filter.clone())
        .and_then(routes::fournisseur::get_dossier);


    //dossier fournisseur resources

    let add_dossier_fournisseur = warp::post() 
        .and(warp::path("dossiers-fournisseurs"))
        .and(warp::path::end())
        .and(dossier_fournisseur_store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::dossier_fournisseur::add_dossier_fournisseur);


    let update_dossier_fournisseur = warp::put()
        .and(warp::path!("dossiers-fournisseurs" / String))
        .and(warp::path::end())
        .and(dossier_fournisseur_store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::dossier_fournisseur::update_dossier_fournisseur);


    let get_dossier_fournisseur = warp::get()
        .and(warp::path!("fournisseurs" / String))
        .and(warp::path::end())
        .and(dossier_fournisseur_store_filter.clone())
        .and_then(routes::dossier_fournisseur::get_dossier_fournisseur);

    let get_dossiers_fournisseurs = warp::get()
        .and(warp::path!("fournisseurs"))
        .and(warp::path::end())
        .and(warp::query())
        .and(dossier_fournisseur_store_filter.clone())
        .and_then(routes::dossier_fournisseur::get_dossiers_fournisseurs);


    let delete_dossier_fournisseur = warp::delete()
        .and(warp::path!("fournisseurs" / String))
        .and(warp::path::end())
        .and(dossier_fournisseur_store_filter.clone())
        .and_then(routes::dossier_fournisseur::delete_fournisseur);


    let registration = warp::post() 
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(auth_store_filter)
        .and(warp::body::json())
        .and_then(routes::authentication::register);



    let routes = get_fournisseurs
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
        .or(registration)
        .with(warp::trace::request())
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], app_port)).await;

    Ok(())

}