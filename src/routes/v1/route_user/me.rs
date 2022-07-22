use crate::routes::{route_error::RouteError, v1::auth_principal::UserPrincipal};
use actix_http::{http::StatusCode, ResponseBuilder};
use actix_web::{get, HttpRequest, HttpResponse};

#[get("/me")]
async fn me(_: HttpRequest, user_principal: UserPrincipal) -> Result<HttpResponse, RouteError> {
    Ok(ResponseBuilder::new(StatusCode::OK).json2(&user_principal.user))
}
