use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use serde::Deserialize;
use validator::Validate;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Deserialize, Debug, Validate)]
pub struct MasterPassword {
    #[validate(length(min = 1, max = 10))]
    db_name: String,
    #[validate(length(min = 1, max = 60))]
    password: String,
}

pub fn auth_master_password(db: &SqliteConnection, data: MasterPassword) -> Result<String, String> {
    Ok("authenticated".to_string())
}

#[derive(Deserialize, Debug, Validate)]
pub struct NewCredential {
    #[validate(length(min = 1, max = 60))]
    title: String,
    #[validate(length(max = 60))]
    username: String,
    #[validate(length(max = 60))]
    password: String,
    #[validate(url)]
    url: String,
}

pub fn add_credential(db: &SqliteConnection, data: NewCredential) -> Result<String, String> {
    println!("{:?}", data);
    Ok("credential added".to_string())
}
