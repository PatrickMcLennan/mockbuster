use actix_web::{
    get,
    web::{Data, Query},
    Error as ActixError, HttpResponse,
};
use sea_orm::DatabaseConnection;
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

use crate::operations::ratings;
use recently_rented_view::recently_rented_view::{Props, RecentlyRented};
use validators::ratings::recently_rented_dto::RecentlyRentedDTO;

const PAGE_SIZE: u64 = 20;

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

    let start_cursor = (page - 1) * PAGE_SIZE;
    let end_cursor = page * PAGE_SIZE;

    let recently_rented =
        ratings::list::execute(start_cursor, end_cursor, db.get_ref().clone()).await;

    println!("{:?}", recently_rented.clone());

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
