use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Session {
    pub exp: DateTime<Utc>,
    pub account_id: AccountId,
    pub nbf: DateTime<Utc>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub id: Option<AccountId>, 
    pub email: String,
    pub password: String,
    //comma separated user roles
    pub roles: Option<String>, 
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountId(pub String);

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Roles{
    DAFP,
    ADMIN,
    CE
}

impl Roles{
    pub fn as_str(&self)-> &'static str{
        match self{
            Roles::ADMIN => "ADMIN",
            Roles::DAFP => "DAFP",
            Roles::CE => "CE"
        }
    }
}