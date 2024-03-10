use actix_session::Session;

pub fn execute(session: Session) -> () {
    session.purge()
}
