use crate::db::{models::User, schema::users::dsl};
use crate::{routes::route_error::RouteError, utils};
use actix_web::{dev, error::ErrorUnauthorized, web, Error, FromRequest, HttpRequest};
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};
use futures_util::future::{err, ok, Ready};

use super::jwt_auth::LoginClaims;
use super::models::user::UserDto;

// auth principals
pub struct UserPrincipal {
    pub user: UserDto,
}

impl UserPrincipal {
    pub fn try_new(claims: &LoginClaims, pool: &utils::DbConnPool) -> Result<Self, RouteError> {
        // fetch user
        let user = dsl::users
            .filter(dsl::id.eq(&claims.id))
            .first::<User>(&pool.get()?)?;

        // return principle
        Ok(UserPrincipal {
            user: UserDto::new(user),
        })
    }
}

// macro to implement FromRequest for principal types
macro_rules! principal_extractor {
    ($pt: ty, $claim: ty) => {
        impl FromRequest for $pt {
            type Error = Error;

            type Future = Ready<Result<Self, Self::Error>>; // future that is immediately ready with a value

            type Config = ();

            fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
                // connection pool must be on web::Data<T> i.e shared state
                let pool = req.app_data::<web::Data<utils::DbConnPool>>().unwrap();
                req.extensions_mut()
                    .remove::<$claim>()
                    .and_then(|claims| <$pt>::try_new(&claims, &pool).ok())
                    .map_or(err(ErrorUnauthorized("Unauthorized")), |p| ok(p))
            }
        }
    };
}

principal_extractor!(UserPrincipal, LoginClaims);
