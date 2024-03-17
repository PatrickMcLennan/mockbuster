use actix_files::Files;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App as ActixApp, HttpServer};
use env_logger::Env;
use sea_orm::{Database, DatabaseConnection};
use std::env;

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;

mod operations;
mod routes;

use routes::{home, login, logout, movie, profile, recently_rented, search, top_ten};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    dotenv::from_path("./../.env").ok();

    let secret_key = Key::generate();
    let redis_url = env::var("REDIS_URL").expect("NO_REDIS_URL_IN_ENV");

    let http_client = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::default(),
            options: HttpCacheOptions::default(),
        }))
        .build();

    let pool: DatabaseConnection =
        Database::connect(env::var("DATABASE_URL").expect("NO_POSTGRES_URL_IN_ENV"))
            .await
            .unwrap();

    let redis_store = match RedisSessionStore::new(redis_url.to_string()).await {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };

    let redis_connection = redis::Client::open(redis_url).unwrap();

    let redis_connection_pool = r2d2::Pool::builder().build(redis_connection).unwrap();

    HttpServer::new(move || {
        ActixApp::new()
			// Logger
			.wrap(Logger::new("\n[IP]: %a \n[Request]: %r\n[User Agent]: %{User-Agent}i \n[Start]: %t\n[Duration]: %T\n[Response]: %s"))
			// Redis connection
			.wrap(
				SessionMiddleware::new(
					redis_store.clone(),
                    secret_key.clone(),
				)
			)
			// Postgres connection pool
			.app_data(web::Data::new(pool.clone()))
			// HTTP Client connection pool
			.app_data(web::Data::new(http_client.clone()))
			// Redis connection pool
			.app_data(web::Data::new(redis_connection_pool.clone()))
			// Static Files
			.service(Files::new("/assets", "./assets/").show_files_listing())
			// Routes
			.service(home::get::get)
			.service(login::get::get)
			.service(login::post::post)
			.service(logout::post::post)
			.service(movie::get::get)
			.service(movie::post::post)
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
