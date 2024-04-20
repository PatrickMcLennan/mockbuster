use actix_session::Session;
use actix_web::{
    post,
    web::{Data, Json, Redirect},
    Error as ActixError,
};
use actix_web_flash_messages::FlashMessage;
use sea_orm::DatabaseConnection;
use validators::users::login_form::LoginFormSchema;

use operations::users;

#[post("/login")]
async fn post(
    Json(form): Json<LoginFormSchema>,
    db: Data<DatabaseConnection>,
    session: Session,
) -> Result<Redirect, ActixError> {
    match form.get_errors() {
        Some(_) => {
            FlashMessage::error("An email and a password are required.").send();
            return Ok(Redirect::to("/login").see_other());
        }
        None => (),
    };

    match users::login::execute(session, db.get_ref().clone(), form).await {
        Some(v) => match v {
            Ok(_) => Ok(Redirect::to("/").see_other()),
            Err(_) => {
                FlashMessage::error("mockuster is down right now.  Please try again later.").send();
                return Ok(Redirect::to("/login").see_other());
            }
        },
        None => {
            FlashMessage::error("Incorrect email or password.  Please try again.").send();
            return Ok(Redirect::to("/login").see_other());
        }
    }
}
