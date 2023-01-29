// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::connection::SimpleConnection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::error::{Error, ErrorKind};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

const TABLE_SCHEMAS: &str = include_str!("../data/create_tables.sql");

/// Create postgres database connection pool.
///
/// # Errors
///
/// Returns error if:
/// - No `DATABASE_URL` is set in current environment.
/// - Failed to connect to database.
pub fn get_connection_pool() -> Result<DbPool, Error> {
    let url = std::env::var("DATABASE_URL").map_err(|err| {
        Error::from_string(
            ErrorKind::ConfigError,
            format!("DATABASE_URL is not set in environment, err: {err:?}"),
        )
    })?;
    let manager = ConnectionManager::<PgConnection>::new(&url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .map_err(|err| {
            Error::from_string(
                ErrorKind::DbConnError,
                format!("Failed to create connection pool, url: {url}, err: {err:?}"),
            )
        })
}

pub fn create_table_schema(conn: &mut PgConnection) -> Result<(), Error> {
    conn.batch_execute(TABLE_SCHEMAS)?;
    Ok(())
}
