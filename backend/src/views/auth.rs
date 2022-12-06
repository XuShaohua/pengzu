// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::cookie::time::OffsetDateTime;
use actix_web::dev::ServiceRequest;
use actix_web::HttpRequest;
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use shared::users::UserRole;

use crate::error::{Error, ErrorKind};
use crate::settings::get_jwt_secret;

pub const TOKEN_NAME: &str = "Token";
const JWT_EXPIRATION_HOURS: i64 = 24 * 3;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserPermissions {
    pub id: i32,
    pub name: String,
    pub role: UserRole,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Claims {
    id: i32,
    name: String,
    role: UserRole,
    exp: i64,
}

impl Claims {
    pub fn new(permission: &UserPermissions) -> Self {
        Self {
            id: permission.id,
            name: permission.name.clone(),
            role: permission.role,
            exp: (Utc::now() + Duration::hours(JWT_EXPIRATION_HOURS)).timestamp(),
        }
    }

    #[must_use]
    pub const fn id(&self) -> i32 {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn role(&self) -> UserRole {
        self.role
    }

    #[must_use]
    pub fn exp_offset(&self) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(self.exp).unwrap()
    }

    #[must_use]
    pub fn permission(&self) -> UserPermissions {
        UserPermissions {
            id: self.id,
            name: self.name.clone(),
            role: self.role,
        }
    }

    pub fn roles(&self) -> Vec<UserRole> {
        match self.role {
            UserRole::Nil => vec![],
            UserRole::User => vec![UserRole::User],
            UserRole::Admin => vec![UserRole::Admin, UserRole::User],
        }
    }

    pub fn decode(token: &str) -> Result<Self, Error> {
        let secret = get_jwt_secret()?;

        let token_data = decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;

        if token_data.claims.role.is_valid() {
            Ok(token_data.claims)
        } else {
            Err(Error::from_string(
                ErrorKind::AuthFailed,
                format!("Invalid user role: {:?}", token_data.claims.role),
            ))
        }
    }

    pub fn encode(&self) -> Result<String, Error> {
        let secret = get_jwt_secret()?;

        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(Into::into)
    }
}

pub async fn auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    // We just get permissions from JWT
    match Claims::decode(credentials.token()) {
        Ok(claims) => {
            req.attach(vec![claims.permission()]);
            Ok(req)
        }
        Err(err) => Err((err.into(), req)),
    }
}

pub async fn admin_auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    // We just get permissions from JWT
    match Claims::decode(credentials.token()) {
        Ok(claims) => {
            req.attach(vec![claims.permission()]);
            Ok(req)
        }
        Err(err) => Err((err.into(), req)),
    }
}

pub fn get_claims_from_auth(req: &HttpRequest) -> Result<Claims, Error> {
    let header = req.headers().get("Authorization").unwrap();
    let invalid_token_error = Error::from_string(
        ErrorKind::InvalidToken,
        format!("invalid token: {:?}", header),
    );

    if header.len() < 8 {
        return Err(invalid_token_error);
    }
    let token = header.to_str().map_err(|_| invalid_token_error.clone())?;
    let mut parts = token.splitn(2, ' ');
    match parts.next() {
        Some(scheme) if scheme == "Bearer" => {}
        _ => return Err(invalid_token_error),
    }
    let token = parts.next().ok_or(invalid_token_error)?;
    Claims::decode(token)
}

pub fn get_claims_from_cookie(req: &HttpRequest) -> Result<Claims, Error> {
    req.cookie("Token").map_or_else(
        || Err(Error::new(ErrorKind::InvalidToken, "invalid token")),
        |cookie| Claims::decode(cookie.value()),
    )
}
