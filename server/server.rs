use actix_files::Files;
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App as ActixApp, HttpServer};
use env_logger::Env;
use sea_orm::{Database, DatabaseConnection};
use std::env;

mod operations;
mod routes;

use routes::{home, login, logout, movie, profile, recently_rented, search, top_ten};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    dotenv::from_path("./../.env").ok();

    let secret_key = Key::generate();
    let http_client = reqwest::Client::new();
    let pool: DatabaseConnection =
        Database::connect(env::var("DATABASE_URL").expect("NO_POSTGRES_URL_IN_ENV"))
            .await
            .unwrap();

    HttpServer::new(move || {
        ActixApp::new()
			// Logger
			.wrap(Logger::new("\n[IP]: %a \n[Request]: %r\n[User Agent]: %{User-Agent}i \n[Start]: %t\n[Duration]: %T\n[Response]: %s"))
			// Redis connection
			.wrap(
				SessionMiddleware::new(
					RedisActorSessionStore::new(env::var("REDIS_URL").expect("NO_REDIS_URL_IN_ENV")),
					secret_key.clone()
				)
			)
			// Postgres connection pool
			.app_data(web::Data::new(pool.clone()))
			// HTTP Client connection pool
			.app_data(web::Data::new(http_client.clone()))
			// Static Files
			.service(Files::new("/assets", "./assets/").show_files_listing())
			// Routes
			.service(home::get::get)
			.service(login::get::get)
			.service(login::post::post)
			.service(logout::post::post)
			.service(movie::get::get)
			.service(profile::get::get)
			.service(recently_rented::get::get)
			.service(search::get::get)
			.service(search::post::post)
			.service(top_ten::get::get)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
