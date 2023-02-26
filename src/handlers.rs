use actix_web::{get, post, web, Error, HttpResponse};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use uuid::Uuid;
use crate::actions;
use crate::models;


type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;


/// Finds user by UID.
#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::find_user_by_uid(&mut conn, user_uid)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid: {user_uid}"));
        Ok(res)
    }
}

/// Inserts new user with name defined in form.
#[post("/user")]
async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::insert_new_user(&mut conn, &form.name)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}
