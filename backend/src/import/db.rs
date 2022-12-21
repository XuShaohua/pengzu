// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use r2d2::Pool;
use std::path::Path;

use crate::error::{Error, ErrorKind};

pub type CalibreDbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn get_calibre_db(calibre_path: &str) -> Result<CalibreDbPool, Error> {
    let calibre_dir = Path::new(calibre_path);
    let db_file = calibre_dir.join("metadata.db");
    let db_file = db_file.into_os_string();
    let db_file: String = db_file.into_string()?;
    let manager = ConnectionManager::<SqliteConnection>::new(&db_file);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .map_err(|err| {
            Error::from_string(
                ErrorKind::DbConnError,
                format!("Failed to open calibre db: {db_file:?}, err: {err:?}"),
            )
        })
}
