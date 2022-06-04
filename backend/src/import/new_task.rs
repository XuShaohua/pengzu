// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::db;
use diesel::{PgConnection, SqliteConnection};

use crate::error::{Error, ErrorKind};
use crate::import::db::get_calibre_db;

fn import_authors(sqlite_conn: &SqliteConnection, pg_conn: &PgConnection) -> Result<(), Error> {
    use crate::models::authors::{add_author, NewAuthor};
    use calibre::models::authors::get_authors;

    let limit = 10;
    let mut offset = 0;
    loop {
        let author_list = get_authors(sqlite_conn, limit, offset)?;
        println!("author list: {:#?}", author_list);
        if author_list.is_empty() {
            break;
        }
        offset += author_list.len() as i64;
        println!("offset: {:?}", offset);

        for author in author_list {
            let new_author = NewAuthor {
                sort: author.sort.unwrap_or_else(|| author.name.clone()),
                name: author.name,
                link: author.link,
            };

            if let Err(err) = add_author(pg_conn, &new_author) {
                match err.kind() {
                    ErrorKind::DbUniqueViolationError => {
                        log::info!("author exists: {:?}", new_author);
                        continue;
                    }
                    _ => return Err(err),
                }
            }
        }
    }

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
    let pg_pool = db::get_connection_pool()?;
    let sqlite_conn = calibre_pool.get()?;
    let pg_conn = pg_pool.get()?;
    import_authors(&sqlite_conn, &pg_conn)?;
    // import_languages(&sqlite_conn)?;
    // import_publishers(&sqlite_conn)?;
    // import_tags(&sqlite_conn)?;

    Ok(())
}
