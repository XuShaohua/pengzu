// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use shared::users::{LoginForm, NewUserReq, UserInfo};

use crate::error::Error;
use crate::models::auth::{self, AuthError};
use crate::schema::users;

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

fn user_to_user_info(user: User) -> UserInfo {
    UserInfo {
        id: user.id,
        name: user.name,
        display_name: user.display_name,
        email: user.email,
        role: user.role.into(),
        created: user.created,
        token: String::new(),
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub role: i32,
    pub hash: String,
    pub salt: String,
}

pub fn add_user(conn: &mut PgConnection, new_user_req: NewUserReq) -> Result<UserInfo, Error> {
    let salt = auth::new_salt()?;
    let hash = auth::encrypt(&new_user_req.password, &salt);
    let new_user = NewUser {
        name: new_user_req.name,
        display_name: new_user_req.display_name,
        email: new_user_req.email,
        role: new_user_req.role.into(),
        hash: hash.hex(),
        salt: salt.hex(),
    };
    let user = diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)?;
    Ok(user_to_user_info(user))
}

pub fn get_users(conn: &mut PgConnection) -> Result<Vec<UserInfo>, Error> {
    let user_list = users::table.load(conn)?;
    Ok(user_list.into_iter().map(user_to_user_info).collect())
}

pub fn login(conn: &mut PgConnection, form: &LoginForm) -> Result<UserInfo, AuthError> {
    log::info!("login() {:?}", form);
    if form.username.is_empty() {
        return Err(AuthError::UsernameIsEmpty);
    }
    if form.password.is_empty() {
        return Err(AuthError::PasswordIsEmpty);
    }
    let user = users::table
        .filter(users::name.eq(&form.username))
        .first::<User>(conn)
        .map_err(|err| {
            log::warn!("login failed: {err:?}");
            AuthError::InvalidPair
        })?;

    let hash = auth::PasswordHash::from_string(&user.hash)?;
    let salt = auth::Salt::from_string(&user.salt)?;
    if auth::verify(&form.password, &hash, &salt).is_err() {
        return Err(AuthError::InvalidPair);
    }

    Ok(user_to_user_info(user))
}

pub fn get_user_info(conn: &mut PgConnection, user_id: i32) -> Result<UserInfo, Error> {
    let user = users::table.find(user_id).first::<User>(conn)?;
    Ok(user_to_user_info(user))
}

pub fn get_user_info_by_name(conn: &mut PgConnection, name: &str) -> Result<UserInfo, Error> {
    let user = users::table
        .filter(users::name.eq(name))
        .first::<User>(conn)?;
    Ok(user_to_user_info(user))
}

pub fn delete_user(conn: &mut PgConnection, user_id: i32) -> Result<(), Error> {
    let _user = get_user_info(conn, user_id)?;
    diesel::delete(users::table.find(user_id)).execute(conn)?;
    Ok(())
}
