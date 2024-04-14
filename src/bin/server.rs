use factura::{run, setup_db_connection};
use factura::config;

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {

    dotenv::dotenv().ok();

    let config = config::Config::new().expect("Config can't be set");
    let conn = setup_db_connection(&config).await?;

    //tracing::info!("Q&A service build ID {}", env!("RUST_WEB_DEV_VERSION"));

    run(&config, conn).await;

    Ok(())
}