use sqlx::postgres::{PgPool, PgPoolOptions,PgRow};
use sqlx::Row;

use handle_errors::Error;

use crate::types::{
    account::Account,
    account::AccountId,
    
};

#[derive(Debug, Clone)]
pub struct AuthStore{
    connection: PgPool,
}



impl AuthStore{

    pub async fn new(pool: PgPool) -> Self{
        AuthStore{
            connection: pool
        }

    }

    pub async fn add_account(
        &self, 
        account: Account
    ) -> Result<bool, Error>{

        match sqlx::query("INSERT INTO account (id, email, password)
            VALUES ($1, $2, $3)")
            .bind(account.id.0)
            .bind(account.email)
            .bind(account.password)
            .execute(&self.connection)
            .await
            {
                Ok(_) => Ok(true),
                Err(error) => {
                    tracing::event!(
                        tracing::Level::ERROR,
                        code = error
                            .as_database_error()
                            .unwrap()
                            .code()
                            .unwrap()
                            .parse::<i32>()
                            .unwrap(),

                        db_message = error
                            .as_database_error()
                            .unwrap()
                            .message(),

                        constraint = error
                            .as_database_error()
                            .unwrap()
                            .constraint()
                            .unwrap()
                    );
                    Err(Error::DatabaseQueryError(error))
                }

            }
    }

}