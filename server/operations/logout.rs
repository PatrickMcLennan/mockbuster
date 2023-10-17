use actix_session::Session;

pub fn logout(session: Session) -> () {
    session.purge()
}
