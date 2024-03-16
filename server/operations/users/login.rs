use actix_session::Session;
use models::generated::users;
use sea_orm::{prelude::*, DatabaseBackend, DatabaseConnection, Statement};
use validators::users::login_form::LoginFormSchema;

pub struct LoginResult {
    pub id: i32,
    pub email: String,
}

pub async fn execute(
    session: Session,
    db: DatabaseConnection,
    login_form: LoginFormSchema,
) -> Result<LoginResult, String> {
    println!("Starting execute");
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
            Some(v) => {
                println!("User found");
                match session.insert("id", v.id) {
                    Ok(_) => {
                        return Ok(LoginResult {
                            email: v.email,
                            id: v.id,
                        })
                    }
                    Err(e) => {
                        println!("Error inserting session: {}", e);
                        panic!();
                    }
                }
            }
            None => {
                println!("Incorrect email or password");
                Err("Incorrect email or password".to_string())
            }
        },
        Err(e) => {
            println!("[Error]: {:?}", e);
            return Err("Incorrect email or password".to_string());
        }
    }
}
