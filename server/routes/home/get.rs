use actix_web::{get, Error as ActixError, HttpResponse};
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

use home_view::home_view::Home;

#[get("/")]
async fn get() -> Result<HttpResponse, ActixError> {
    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            yew::ServerRenderer::<Home>::new().render().await
        })
    })
    .await
    .expect("the thread has failed.");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            r#"
			<html lang="en">
				<head>
					<meta charset="UTF-8" />
					<meta http-equiv="X-UA-Compatible" content="IE=edge" />
					<meta name="viewport" content="width=device-width, initial-scale=1.0" />
					<link rel="stylesheet" href="/assets/bootstrap.css" />
					<script defer src="/assets/bootstrap.js" type="text/javascript"></script>
					<script defer src="/assets/homeView.js"></script>
					<title>Home | mockbuster</title>
				</head>
				<body>
					{}
				</body>
			</html>
		"#,
            content
        )))
}
