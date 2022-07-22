use crate::{
    db::{models::User, schema::users::dsl},
    routes::{
        route_error::RouteError,
        v1::{
            jwt_auth::{create_login_token, LoginClaims},
            models::user::UserDto,
        },
    },
    utils,
};
use actix_http::{error::ErrorUnauthorized, http::StatusCode, ResponseBuilder};
use actix_web::{
    post,
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

// Request / Response Dtos
#[derive(Deserialize)]
struct LoginRequest {
    password: String,
    username: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    user: UserDto,
}

// TODO: make database service for common database queries ??


fn fetch_user(login_username: &String, conn: &utils::DbConn) -> Result<User, RouteError> {
    Ok(dsl::users
        .filter(dsl::username.eq(login_username))
        .first::<User>(conn)?)
}

fn check_password(login_password: &String, user: &User) -> Result<(), RouteError> {
    if bcrypt::verify(login_password, &user.password)? {
        Ok(())
    } else {
        Err(RouteError::RouteError(ErrorUnauthorized(
            "Invalid username or password.",
        )))
    }
}

#[post("/login")]
async fn login(
    _: HttpRequest,
    pool: web::Data<utils::DbConnPool>,
    dto: Json<LoginRequest>,
) -> Result<HttpResponse, RouteError> {
    let conn = pool.get()?;
    let req = dto.0;

    // fetch user and check hash
    let user = fetch_user(&req.username, &conn)?;
    check_password(&req.password, &user)?;

    // create token
    let token = create_login_token(&LoginClaims { id: user.id });

    // response
    Ok(ResponseBuilder::new(StatusCode::OK).json2(&LoginResponse {
        user: UserDto {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            username: user.username,
        },
        token: token,
    }))
}
