// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum UserRole {
    User = 0,
    Admin = 2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub role: UserRole,
    pub created: NaiveDateTime,
    pub token: String,
}

// TODO(Shaohua): Replace String with &str.
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}
