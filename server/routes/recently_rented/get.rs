use actix_web::{
    get,
    web::{Data, Query},
    Error as ActixError, HttpResponse,
};
use sea_orm::DatabaseConnection;
use serde_json::Value;
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

use crate::operations::get_recently_rented_movies::get_recently_rented_movies;
use recently_rented_view::recently_rented_view::{Props, RecentlyRented};
use validators::recently_rented_dto::RecentlyRentedDTO;

#[get("/recently-rented")]
async fn get(
    db: Data<DatabaseConnection>,
    params: Query<RecentlyRentedDTO>,
) -> Result<HttpResponse, ActixError> {
    let page = match params.page {
        Some(v) => match v.to_string().parse::<u64>() {
            Ok(page) => page,
            Err(_) => 1,
        },
        None => 1,
    };

    let recently_rented = get_recently_rented_movies(page, db.get_ref().clone()).await;

    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            yew::ServerRenderer::<RecentlyRented>::with_props(move || Props {
                results: Some(recently_rented),
            })
            .render()
            .await
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
					<script defer src="/assets/recentlyRentedView.js"></script>
				</head>
				<body>
					{}
				</body>
			</html>
		"#,
            content
        )))
}
