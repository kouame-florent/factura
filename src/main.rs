
use warp::{http::Method, Filter};
use error::return_error;
use tracing_subscriber::fmt::format::FmtSpan;

mod types;
mod routes;
mod error;
mod repo;

#[tokio::main]
async fn main() {

    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "error=warn,factura=info,warp=error".to_owned());
        

    let db_url = "postgres://factura:factura@localhost:5432/factura";
    let fournisseur_repo = repo::fournisseur::FournisseurRepo::new(db_url).await;

    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes.
        // This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let store_filter = warp::any().map(move || fournisseur_repo.clone());
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
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::fournisseur::add_fournisseur);

    let get_fournisseurs = warp::get()
        .and(warp::path("fournisseurs"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::fournisseur::get_fournisseurs);

   
    let update_fournisseur = warp::put()
        .and(warp::path!("fournisseurs" / String))
        //.and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::fournisseur::update_fournisseur);

    let get_fournisseur = warp::get()
        .and(warp::path!("fournisseurs" / String))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::fournisseur::get_fournisseur);

    let delete_fournisseur = warp::delete()
        .and(warp::path!("fournisseurs" / String))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::fournisseur::delete_fournisseur);
        

    let routes = get_fournisseurs
        .or(update_fournisseur)
        .or(add_fournisseur)
        .or(get_fournisseur)
        .or(delete_fournisseur)
        .with(warp::trace::request())
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

}