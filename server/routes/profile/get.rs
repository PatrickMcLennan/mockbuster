use actix_web::{get, Error as ActixError, HttpResponse};
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

use profile_view::profile_view::Profile;

#[get("/profile")]
async fn get() -> Result<HttpResponse, ActixError> {
    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            let ssr_renderer = yew::ServerRenderer::<Profile>::new();
            let ssr_rendered = ssr_renderer.render().await;
            ssr_rendered
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
					<script defer src="/assets/bootstrap.js"></script>
					<link rel="stylesheet" href="/assets/bootstrap.css" />
					<title>Recently Rented | mockbuster</title>
					<script defer src="/assets/profileView.js"></script>
				</head>
				<body>
					{}
				</body>
			</html>
		"#,
            content
        )))
}