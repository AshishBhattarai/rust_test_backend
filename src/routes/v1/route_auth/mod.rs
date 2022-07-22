use actix_web::{web, Scope};

mod login;
mod signup;

pub fn priv_routes() -> Scope {
    web::scope("auth")
}

pub fn pub_routes() -> Scope {
    web::scope("auth")
        .service(signup::signup)
        .service(login::login)
}
