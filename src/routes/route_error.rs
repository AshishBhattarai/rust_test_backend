use actix_http::ResponseBuilder;
use actix_web::ResponseError;

use derive_more::{Display, Error};

// Convert error for different libs/systems to ResponseError
#[derive(Debug, Display, Error)]
pub enum RouteError {
    #[display(fmt = "Database Connection Pool Error")]
    PoolError(r2d2::Error),
    #[display(fmt = "Database Error")]
    DbError(diesel::result::Error),
    #[display(fmt = "Password Hash Error")]
    BcryptError(bcrypt::BcryptError),
    #[display(fmt = "Generic Route Error")]
    RouteError(actix_web::Error),
}

impl ResponseError for RouteError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            RouteError::PoolError(err) => {
                log::error!("Error: {} Reason: {}", self, err);
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            RouteError::DbError(err) => {
                if let diesel::result::Error::NotFound = err {
                    actix_web::http::StatusCode::NOT_FOUND
                } else if let diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) = err
                {
                    log::error!("Error: {} Reason: {}", self, err);
                    actix_web::http::StatusCode::CONFLICT
                } else {
                    log::error!("Error: {} Reason: {}", self, err);
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
                }
            }
            RouteError::BcryptError(err) => {
                log::error!("Error: {} Reason: {}", self, err);
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            RouteError::RouteError(err) => {
                log::error!("Error: {} Reason: {}", self, err);
                err.as_response_error().status_code()
            }
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        ResponseBuilder::new(self.status_code()).finish()
    }
}

// macro to implement conversion trait From<> for different library error types
macro_rules! error_conversion {
    ($from: ty, $varient: expr ) => {
        impl From<$from> for RouteError {
            fn from(error: $from) -> Self {
                $varient(error)
            }
        }
    };
}

error_conversion!(bcrypt::BcryptError, RouteError::BcryptError);
error_conversion!(diesel::result::Error, RouteError::DbError);
error_conversion!(r2d2::Error, RouteError::PoolError);
