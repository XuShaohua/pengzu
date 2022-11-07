// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

//! Migrate to v0.3.2

use diesel::PgConnection;

use crate::db;
use crate::error::Error;
use crate::models::authors;
use crate::models::tags;

pub fn migrate() -> Result<(), Error> {
    let db_pool = db::get_connection_pool()?;
    let mut pg_conn = db_pool.get()?;

    split_author_names(&mut pg_conn)?;
    split_tag_names(&mut pg_conn)
}

fn split_author_names(conn: &mut PgConnection) -> Result<(), Error> {
    // See import_tags() in src/import/new_task.rs
    let patterns = [" & ", "; ", "；"];
    for pattern in patterns {
        let name_pattern = format!("%{}%", pattern);
        if let Ok(tag) = tags::get_tag_by_name_pattern(conn, &name_pattern) {
            log::info!("find tag: {:?}", tag);
        }
    }
    Ok(())
}

fn split_tag_names(conn: &mut PgConnection) -> Result<(), Error> {
    // See import_authors() in src/import/new_task.rs
    let patterns = [";", "&", "；", "、"];
    for pattern in patterns {
        let name_pattern = format!("%{}%", pattern);
        if let Ok(author) = authors::get_author_by_name_pattern(conn, &name_pattern) {
            log::info!("find author: {:?}", author);
        }
    }
    Ok(())
}
