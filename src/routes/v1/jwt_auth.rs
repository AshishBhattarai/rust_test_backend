use actix_web::{
    dev::{HttpResponseBuilder, ServiceRequest},
    HttpMessage, ResponseError,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, NaiveDateTime, Utc};
use derive_more::{Display, Error};
use hmac::{Hmac, NewMac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use uuid::Uuid;

use crate::utils::string_functions;

/**
 * JWT claims
 */
#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct LoginClaims {
    pub id: Uuid,
}
// Dummy for now
#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct SignupClaims {
    pub id: Uuid,
}

// A macro to automatically create claim enum and add common fields(exp) to each claim
macro_rules! claim_enum {
    ($($name:ident : $type: ty),+) => { // takes in comma separated NAME : TYPE varadic(+)
        #[derive(Deserialize, Serialize)]
        #[serde(rename_all = "lowercase", tag = "type")]
        enum Claim {
            $($name {
               claims: $type,
               exp: NaiveDateTime
            }),+ // unpack
        }
    };
}

claim_enum!(Singup: SignupClaims, Login: LoginClaims);

/**
 * JWTAuth for handeling token creation and validation
 */
pub struct JWTAuth {
    secret_key: Hmac<Sha256>,
    header: Header,
}

impl JWTAuth {
    // S must be convertable to String ie will support &str
    fn new<S: Into<String>>(secret_key: S) -> JWTAuth {
        JWTAuth {
            secret_key: NewMac::new_varkey(secret_key.into().as_bytes())
                .expect("[Server Config] - Invalid secret key."),
            header: Header {
                algorithm: AlgorithmType::Hs256,
                ..Default::default()
            },
        }
    }

    pub fn login(&self, token: &str) -> Result<LoginClaims, AuthError> {
        let verified_token: Token<Header, Claim, _> =
            VerifyWithKey::verify_with_key(token, &self.secret_key)?;
        if let Claim::Login { exp, claims } = verified_token.claims() {
            check_exp(exp).map(|_| claims.clone())
        } else {
            Err(AuthError::InvalidToken)
        }
    }

    pub fn login_token(&self, login_claims: &LoginClaims) -> String {
        // TODO: proper expire handeling
        String::from(
            Token::new(
                Header {
                    algorithm: self.header.algorithm,
                    ..Default::default()
                },
                Claim::Login {
                    claims: *login_claims,
                    exp: (Utc::now() + Duration::days(1)).naive_utc(),
                },
            )
            .sign_with_key(&self.secret_key)
            .unwrap()
            .as_str(),
        )
    }
}

fn check_exp(exp: &NaiveDateTime) -> Result<(), AuthError> {
    if Utc::now().naive_utc().lt(exp) {
        Ok(())
    } else {
        Err(AuthError::ExpiredToken)
    }
}

/**
 * AuthError
 */
#[derive(Debug, Display, Error)]
pub enum AuthError {
    #[display(fmt = "Invalid Token")]
    InvalidToken,
    #[display(fmt = "Expired Token")]
    ExpiredToken,
}

impl From<jwt::Error> for AuthError {
    fn from(_: jwt::Error) -> Self {
        Self::InvalidToken
    }
}

lazy_static! {
    pub static ref AUTH: JWTAuth = JWTAuth::new(string_functions::get_evn_or_panic("JWT_SECRET"));
}

/**
 * JWT validator for middleware for actix
 **/
impl ResponseError for AuthError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::UNAUTHORIZED
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponseBuilder::new(self.status_code()).finish()
    }
}

pub async fn validator(
    req: ServiceRequest,
    creds: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    let claims = AUTH.login(creds.token())?;
    req.extensions_mut().insert(claims);
    Ok(req)
}

pub fn create_login_token(claims: &LoginClaims) -> String {
    AUTH.login_token(claims)
}
