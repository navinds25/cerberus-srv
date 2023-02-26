#[cfg(test)]
mod tests {
    use actix_web::{test, App, middleware, web};
    use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
    use crate::handlers;
    use crate::models;

    #[actix_web::test]
    async fn user_routes() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
        env_logger::init();
        dotenv::dotenv().ok();

        let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(middleware::Logger::default())
                .service(handlers::get_user)
                .service(handlers::add_user),
        )
        .await;

        // Insert a user
        let req = test::TestRequest::post()
            .uri("/user")
            .set_json(&models::NewUser {
                name: "Test user".to_owned(),
            })
            .to_request();

        let resp: models::User = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.name, "Test user");

        // Get a user
        let req = test::TestRequest::get()
            .uri(&format!("/user/{}", resp.id))
            .to_request();

        let resp: models::User = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.name, "Test user");

        // Delete new user from table
        use crate::schema::users::dsl::*;
        diesel::delete(users.filter(id.eq(resp.id)))
            .execute(&mut pool.get().expect("couldn't get db connection from pool"))
            .expect("couldn't delete test user from table");
    }
}
