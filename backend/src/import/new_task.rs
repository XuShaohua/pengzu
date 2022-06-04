// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use diesel::SqliteConnection;

use crate::error::Error;
use crate::import::db::get_calibre_db;

fn import_authors(conn: &SqliteConnection) -> Result<(), Error> {
    use calibre::models::authors::get_authors;
    let limit = 10;
    let mut offset = 0;
    let author_list = get_authors(conn, limit, offset)?;
    println!("author list: {:#?}", author_list);
    offset += author_list.len() as i64;
    println!("offset: {:?}", offset);

    Ok(())
}

fn import_languages(conn: &SqliteConnection) -> Result<(), Error> {
    use calibre::models::languages::get_languages;
    let lang_list = get_languages(conn)?;
    println!("lang list: {:#?}", lang_list);

    Ok(())
}

fn import_publishers(conn: &SqliteConnection) -> Result<(), Error> {
    use calibre::models::publishers::get_publishers;
    let limit = 10;
    let mut offset = 0;
    let publisher_list = get_publishers(&conn, limit, offset)?;
    println!("publishers: {:#?}", publisher_list);
    offset += publisher_list.len() as i64;
    println!("offset: {}", offset);
    Ok(())
}

fn import_tags(conn: &SqliteConnection) -> Result<(), Error> {
    use calibre::models::tags::get_tags;
    let limit = 10;
    let mut offset = 0;
    let tag_list = get_tags(conn, limit, offset)?;
    println!("tags: {:#?}", tag_list);
    offset += tag_list.len() as i64;
    println!("offset: {}", offset);

    Ok(())
}

pub fn new_task(calibre_path: &str) -> Result<(), Error> {
    let calibre_pool = get_calibre_db(calibre_path)?;
    println!("calibre pool: ");
    let conn = calibre_pool.get()?;
    import_authors(&conn)?;
    import_languages(&conn)?;
    import_publishers(&conn)?;
    import_tags(&conn)?;

    Ok(())
}
