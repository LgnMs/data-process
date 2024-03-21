use std::env;
use std::fmt::Display;
use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::{FromRequestParts, Request};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, RequestPartsExt, Router};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::api::common::{AppState, ResJson};
use crate::data_response;

pub fn set_routes() -> Router<Arc<AppState>> {
    let routes = Router::new().route("/authorize", post(authorize));

    routes
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET is not set in .env file");
    Keys::new(secret.as_bytes())
});

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

// TODO 接入认证信息
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Claims {
    auth_id: String,
    auth_secret: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    auth_id: String,
    auth_secret: String,
}

#[derive(Debug)]
pub enum AuthError {
    #[allow(dead_code)]
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        if parts.uri.path() == "/auth/authorize" {
            return Ok(Claims::default());
        }
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "auth_id: {}\nauth_secret: {}",
            self.auth_id, self.auth_secret
        )
    }
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

// - get an authorization token:
//
// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -d '{"client_id":"foo","client_secret":"bar"}' \
//     http://localhost:3000/authorize
//
// - visit the protected area using the authorized token
async fn authorize(Json(payload): Json<AuthPayload>) -> Result<ResJson<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.auth_id.is_empty() || payload.auth_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // TODO 接入认证信息逻辑
    // if payload.auth_id != "foo" || payload.auth_secret != "bar" {
    //     return Err(AuthError::WrongCredentials);
    // }
    let claims = Claims {
        auth_id: payload.auth_id,
        auth_secret: payload.auth_secret,
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };

    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    // Ok(Json(AuthBody::new(token)))
    let res: anyhow::Result<AuthBody> = Ok(AuthBody::new(token));
    data_response!(res)
}

// Claims 作为extract 在没有提取到对应的token时抛出错误
pub async fn jwt_middleware(
    _: Claims,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let response = next.run(request).await;

    Ok(response)
}
