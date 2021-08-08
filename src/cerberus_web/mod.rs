use actix_web::{web, HttpResponse};
pub mod password_store;

/* health check endpoint */
pub async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"health\": \"ok\"}")
}

/* app_config handles the routing for all endpoints */
pub fn app_config(config: &mut web::ServiceConfig) {
    let health_resource = web::resource("/v1/health").route(web::get().to(health));
    config.service(health_resource);
    /* password store resources: */
    let auth_master_password = web::resource("/v1/ps/auth").route(web::post().to(password_store::auth_master_password));
    config.service(auth_master_password);
    let add_credential = web::resource("/v1/ps/credential").route(web::post().to(password_store::add_credential));
    config.service(add_credential);
}
