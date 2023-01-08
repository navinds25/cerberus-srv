use actix_web::{App, HttpServer};
use diesel::r2d2::ConnectionManager;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use std::process::exit;
mod cerberus_web;

#[derive(Serialize, Deserialize)]
struct Config {
    address: String,
    database_url: String,
}

gflags::define! {
    --websrv-config: &Path
}

#[actix_web::main]
async fn startsrv(config: Config) -> std::io::Result<()> {
    println!("starting server on address: http://{}", config.address);
    /* database setup */
    let database_pool = cerberus_web::password_store::db::DbPool::builder()
        .build(ConnectionManager::new(config.database_url))
        .expect("Failed to create DB pool");
    /* start HTTP Server */
    return HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            .configure(cerberus_web::app_config_v1)
            //.configure(cerberus_web::app_config_v2)
    })
    .bind(config.address)?
    .run()
    .await;
}

fn main() -> std::io::Result<()> {
    gflags::parse();
    /* default config values */
    let mut config = Config {
        address: "127.0.0.1:5000".to_string(),
        database_url: "default.db".to_string(),
    };
    /* parse provided config */
    if WEBSRV_CONFIG.is_present() {
        let config_filename = WEBSRV_CONFIG.flag;
        if !config_filename.exists() {
            println!("can't find config file: {}", config_filename.display());
            exit(1);
        }
        let config_file = File::open(config_filename).expect("Unable to open config file");
        config = serde_json::from_reader(config_file).expect("Can't parse json config");
        println!("address: {}", config.address);
        println!("database_url: {}", config.database_url);
    }
    return startsrv(config);
}
