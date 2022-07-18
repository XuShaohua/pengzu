// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

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
    pub id: i32,
    pub name: String,
    pub role: UserRole,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn role(&self) -> UserRole {
        self.role
    }

    pub fn roles(&self) -> Vec<UserRole> {
        match self.role {
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
