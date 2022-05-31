// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::error::{Error, ErrorKind};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> Result<DbPool, Error> {
    let url = std::env::var("DATABASE_URL").map_err(|err| {
        Error::from_string(
            ErrorKind::ConfigError,
            format!("DATABASE_URL is not set in environment, err: {:?}", err),
        )
    })?;
    let manager = ConnectionManager::<PgConnection>::new(url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .map_err(|err| {
            Error::from_string(
                ErrorKind::DbError,
                format!(
                    "Failed to create connection pool, url: {}, err: {:?}",
                    url, err
                ),
            )
        })
}
