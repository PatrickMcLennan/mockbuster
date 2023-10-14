use actix_files::Files;
use actix_web::{middleware::Logger, App as ActixApp, HttpServer};

mod routes;

use routes::{home, login};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        ActixApp::new()
			.wrap(Logger::new("%a \n[Request]: %r\n[User Agent]: %{User-Agent}\n[Start]: %t\n[Duration]: %T\n[Response]: %s i"))
			.service(Files::new("/assets", "./assets/dist"))
			.service(login::get::get)
			.service(login::post::post)
			.service(home::get::get)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
