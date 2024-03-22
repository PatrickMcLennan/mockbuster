use actix_session::{Session, SessionInsertError};
use models::generated::users;
use sea_orm::{prelude::*, DatabaseBackend, DatabaseConnection, Statement};
use validators::users::login_form::LoginFormSchema;

pub struct LoginResult {
    pub id: i32,
    pub email: String,
}

const LOG_KEY: &str = "[Operations::Users::Login]: ";

pub async fn execute(
    session: Session,
    db: DatabaseConnection,
    login_form: LoginFormSchema,
) -> Option<Result<LoginResult, DbErr>> {
    match users::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "SELECT * FROM users WHERE email = $1 AND password_hash = crypt($2, password_hash)",
            [login_form.email.into(), login_form.password.into()],
        ))
        .one(&db)
        .await
    {
        Ok(record_option) => match record_option {
            Some(v) => match session.insert("id", v.id) {
                Ok(_) => Some(Ok(LoginResult {
                    email: v.email,
                    id: v.id,
                })),
                Err(e) => {
                    println!("{}{:?}", LOG_KEY, e);
                    None
                }
            },
            None => None,
        },
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Some(Err(e))
        }
    }
}
