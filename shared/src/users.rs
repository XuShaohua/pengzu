// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum UserRole {
    Nil = 0,
    User = 1,
    Admin = 2,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::Nil
    }
}

#[cfg(feature = "use_query")]
impl diesel::Expression for UserRole {
    type SqlType = diesel::sql_types::Integer;
}

impl From<i32> for UserRole {
    fn from(role: i32) -> Self {
        match role {
            1 => Self::User,
            2 => Self::Admin,
            _ => Self::Nil,
        }
    }
}

impl From<UserRole> for i32 {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::Nil => 0,
            UserRole::User => 1,
            UserRole::Admin => 2,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub role: UserRole,
    pub created: NaiveDateTime,
    pub token: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}
