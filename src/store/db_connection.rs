use sqlx::postgres::{PgPool, PgPoolOptions,};


#[derive(Debug, Clone)]
pub struct DBConnection{
    pub pool: PgPool,
}

impl DBConnection{
    pub async fn new(db_url: &str) -> Self{
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await {
                Ok(pool) => pool,
                Err(e) => panic!("Couldn't establish DB connection: {}", e)
            };
        
        DBConnection{
            pool: db_pool,
        }
    }

}