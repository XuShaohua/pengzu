// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::Error;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum UserRole {
    User = 0,
    Admin = 2,
}

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub role: i32,
    pub salt: String,
    pub hash: String,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserResp {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub role: i32,
    pub created: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub role: UserRole,
    pub hash: String,
    pub salt: String,
}

pub fn add_user() -> Result<UserResp, Error> {
    todo!()
}

pub fn get_all_users() -> Result<Vec<User>, Error> {
    todo!()
}

pub fn delete_user() -> Result<(), Error> {
    todo!()
}
