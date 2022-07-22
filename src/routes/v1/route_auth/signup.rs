use crate::{db::schema::users, routes::route_error::RouteError, utils};
use actix_http::{http::StatusCode, ResponseBuilder};
use actix_web::{
    post,
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use bcrypt::DEFAULT_COST;
use diesel::RunQueryDsl;
use serde::Deserialize;

// Inserted on singup API, chaing this will affect the insert
#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct Signup {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

fn insert_new_user(mut new_user: Signup, conn: &utils::DbConn) -> Result<(), RouteError> {
    // hash password
    new_user.password = bcrypt::hash(&new_user.password, DEFAULT_COST)?;

    // insert user
    diesel::insert_into(users::dsl::users)
        .values(&new_user)
        .execute(conn)?;
    Ok(())
}

#[post("/signup")]
async fn signup(
    _: HttpRequest,
    pool: web::Data<utils::DbConnPool>,
    dto: Json<Signup>,
) -> Result<HttpResponse, RouteError> {
    let conn = pool.get()?;

    insert_new_user(dto.0, &conn)?;

    Ok(ResponseBuilder::new(StatusCode::CREATED).finish())
}
