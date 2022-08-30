// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::cookie::time::OffsetDateTime;
use actix_web::dev::ServiceRequest;
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::models::users::UserRole;
use crate::settings::get_jwt_secret;

pub const TOKEN_NAME: &str = "Token";
const JWT_EXPIRATION_HOURS: i64 = 24 * 3;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UserPermissions {
    pub user_id: i32,
    pub name: String,
    pub role: UserRole,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Claims {
    user_id: i32,
    name: String,
    role: UserRole,
    exp: i64,
}

impl Claims {
    pub fn new(permission: &UserPermissions) -> Self {
        Self {
            user_id: permission.user_id,
            name: permission.name.clone(),
            role: permission.role,
            exp: (Utc::now() + Duration::hours(JWT_EXPIRATION_HOURS)).timestamp(),
        }
    }

    #[must_use]
    pub const fn user_id(&self) -> i32 {
        self.user_id
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

    pub fn permission(self) -> UserPermissions {
        UserPermissions {
            user_id: self.user_id,
            name: self.name,
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

        Ok(token_data.claims)
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
            log::info!("auth_validator() claims: {:?}", claims);
            req.attach(vec![claims.permission()]);
            Ok(req)
        }
        Err(err) => Err((err.into(), req)),
    }
}
