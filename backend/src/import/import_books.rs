// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{PgConnection, SqliteConnection};

use crate::error::Error;

pub fn import_books(
    _calibre_path: &str,
    _sqlite_conn: &SqliteConnection,
    _pg_conn: &PgConnection,
) -> Result<(), Error> {
    Ok(())
}
