#[macro_use]
extern crate diesel;

use actix_web::{middleware::Logger, App, HttpServer};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use utils::{string_functions, DbConnPool};

mod db;
mod routes;
mod utils;

fn setup_database(database_url: &String) -> DbConnPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create database pool.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load .env
    dotenv::dotenv().expect("Failed to load .env");

    // configs
    let database_url = string_functions::get_evn_or_panic("DATABASE_URL");

    // https://docs.rs/env_logger/0.8.3/env_logger/
    env_logger::init();

    // setup database connection pool
    let pool = setup_database(&database_url);

    // steup and start http server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" %T"))
            .data(pool.clone())
            .configure(routes::setup_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
