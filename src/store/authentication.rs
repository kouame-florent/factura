use sqlx::postgres::{PgPool,PgRow};
use sqlx::Row;
use tracing::instrument;

use handle_errors::Error;

use crate::types::account::{NewAccount, Roles};
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
        new_account: NewAccount
    ) -> Result<bool, Error>{

        match sqlx::query("INSERT INTO account (id, email, password, roles)
            VALUES ($1, $2, $3, $4)")
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(new_account.email)
            .bind(new_account.password)
            .bind(new_account.roles)
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

    #[instrument]
    pub async fn get_email(
        &self, 
        account_id: String,
    ) -> Result<String, Error>{
        match  sqlx::query("SELECT email from account where id = $1")
            .bind(account_id)
            .map(|row: PgRow| row.get("email"))
            .fetch_one(&self.connection)
            .await
            {
                Ok(email) => Ok(email),
                Err(error) => {      
                    tracing::event!(tracing::Level::ERROR, "{:?}", error);
                    Err(Error::DatabaseQueryError(error))
                }
            }
    }

    pub async fn get_account(
            &self, 
            email: String
    ) -> Result<Account, Error> {
        match sqlx::query("SELECT * from account where email = $1")
            .bind(email)
            .map(|row: PgRow| Account {
                id: Some(AccountId(row.get("id"))),
                email: row.get("email"),
                password: row.get("password"),
                roles: row.get("roles")

            })
            .fetch_one(&self.connection)
            .await
            {
                Ok(account) => Ok(account),
                Err(error) => {
                    tracing::event!(tracing::Level::ERROR, "{:?}", error);
                    Err(Error::DatabaseQueryError(error))
                }
            }
    }

    
    pub async fn has_authorization(
        &self,
        account_id: String,
        target_role: String
    ) -> Result<bool, Error>{
        match sqlx::query("SELECT * from account where id = $1") 
        .bind(account_id)
        .map(|row: PgRow| Account {
            id: Some(AccountId(row.get("id"))),
            email: row.get("email"),
            password: row.get("password"),
            roles: row.get("roles")
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(account) => Ok(
                self.check_role(account.roles, target_role).await
            ),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    async fn check_role(
        &self,
        account_roles: Option<String>,
        target_role: String
    ) -> bool{

        let roles: Vec<&str> = match account_roles {
            Some(ref r) => r.split(",").collect(),
            None => vec![] 
        };

       // println!("--- Current roles: {:?}", roles.clone());
        if roles.contains(&Roles::ADMIN.as_str()){
            return true;
        }else{
            for role in roles{
                if role == target_role{
                    return true;
                }
            }
        }
   
        false
    }


   
}