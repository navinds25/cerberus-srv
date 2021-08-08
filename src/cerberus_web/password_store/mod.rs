/* mod.rs contains the handlers for the password_store */

use actix_web::{web, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use serde::Deserialize;
pub mod db;

/* auth_master_password */
pub async fn auth_master_password(
    req: web::Json<db::MasterPassword>,
    db_pool: web::Data<db::DbPool>,
) -> HttpResponse {
    let mpass = req.into_inner();
    println!("{:?}", mpass);
    let conn = db_pool
        .get()
        .expect("could not get connection from db_pool");
    let data = web::block(move || db::auth_master_password(&conn, mpass))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("hello");
    println!("{:?}", data);
    HttpResponse::Ok().json(data)
}

/* add_credential */
pub async fn add_credential(
    req: web::Json<db::NewCredential>,
    db_pool: web::Data<db::DbPool>,
) -> HttpResponse {
    let new_cred = req.into_inner();
    println!("{:?}", new_cred);
    let conn = db_pool.get().expect("couldn't get connection from db_pool");
    let data = web::block(move || db::add_credential(&conn, new_cred))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("hello");
    println!("{:?}", data);
    HttpResponse::Ok().json(data)
}

/* update_credential */

/* remove_credential */
