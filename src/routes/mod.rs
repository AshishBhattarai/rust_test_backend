use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use self::v1::jwt_auth;


//mod response_format;
mod route_error;
mod v1;

pub fn setup_routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("api/v1")
            .wrap(HttpAuthentication::bearer(jwt_auth::validator))
            .service(v1::route_auth::priv_routes())
            .service(v1::route_user::priv_routes()),
    )
    .service(web::scope("pub/api/v1").service(v1::route_auth::pub_routes()));
}
