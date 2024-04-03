
use warp::{http::Method, Filter};
use error::return_error;
use tracing_subscriber::fmt::format::FmtSpan;

mod types;
mod routes;
mod error;
mod store;

#[tokio::main]
async fn main() {

    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "error=warn,factura=info,warp=error".to_owned());
        

    let db_url = "postgres://factura:factura@localhost:5432/factura";
    let conn = store::db_connection::DBConnection::new(db_url).await;

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


    let add_dossier_fournisseur = warp::post() 
        .and(warp::path("dossiers-fournisseurs"))
        .and(warp::path::end())
        .and(dossier_fournisseur_store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::dossier_fournisseur::add_dossier_fournisseur);

        

    let routes = get_fournisseurs
        .or(update_fournisseur)
        .or(add_fournisseur)
        .or(get_fournisseur)
        .or(delete_fournisseur)
        .or(get_dossiers_by_fournisseur_id)
        .or(add_dossier_fournisseur)
        .or(get_specific_dossier_by_fournisseur_id)
        .with(warp::trace::request())
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

}