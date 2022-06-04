// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use diesel::SqliteConnection;

use crate::error::Error;
use crate::import::db::get_calibre_db;

fn import_authors(conn: &SqliteConnection) -> Result<(), Error> {
    use calibre::models::authors;
    let limit = 10;
    let mut offset = 0;
    let author_list = authors::get_authors(conn, limit, offset)?;
    println!("author list: {:#?}", author_list);
    offset += author_list.len() as i64;
    println!("offset: {:?}", offset);

    Ok(())
}

pub fn new_task(calibre_path: &str) -> Result<(), Error> {
    let calibre_pool = get_calibre_db(calibre_path)?;
    println!("calibre pool: ");
    let conn = calibre_pool.get()?;
    import_authors(&conn)?;

    Ok(())
}
