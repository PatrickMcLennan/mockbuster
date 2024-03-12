use crate::operations::users;
use actix_session::Session;
use actix_web::{
    get,
    web::{Data, Query},
    Error as ActixError, HttpResponse,
};
use profile_view::profile_view::Profile;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

#[derive(Serialize, Deserialize)]
struct Params {
    pub id: Option<i32>,
}

#[get("/profile/{profile_id}")]
async fn get(
    db: Data<DatabaseConnection>,
    params: Query<Params>,
    session: Session,
) -> Result<HttpResponse, ActixError> {
    let unauthed = HttpResponse::Unauthorized().finish();

    let id = match params.id {
        Some(id) => id,
        None => match session.get("id") {
            Ok(v) => match v {
                Some(id) => id,
                None => return Ok(unauthed),
            },
            Err(_) => return Ok(unauthed),
        },
    };

    let profile = users::fetch::execute(id, db.get_ref().clone()).await;

    match profile.len() {
        1 => (),
        0 => return Ok(HttpResponse::NotFound().finish()),
        _ => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            yew::ServerRenderer::<Profile>::new().render().await
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
