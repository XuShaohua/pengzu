// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::models::users::UserRole;
use crate::settings::get_jwt_secret;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UserToken {
    pub id: i32,
    pub name: String,
    pub role: UserRole,
    pub exp: usize,
}

impl UserToken {
    pub fn decode(token: &str) -> Result<Self, Error> {
        let secret = get_jwt_secret()?;

        let token_data = decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS512),
        )?;

        Ok(token_data.claims)
    }

    pub fn encode(&self) -> Result<String, Error> {
        let secret = get_jwt_secret()?;

        let header = Header {
            kid: Some("signing_key".to_owned()),
            alg: Algorithm::HS512,
            ..Default::default()
        };
        encode(&header, &self, &EncodingKey::from_secret(secret.as_bytes())).map_err(Into::into)
    }
}
