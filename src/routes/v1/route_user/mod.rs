use actix_web::{web, Scope};

mod me;

pub fn priv_routes() -> Scope {
    web::scope("users").service(me::me)
}

// pub fn pub_routes() -> Scope {
//     web::scope("users")
// }
