// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use calibre::models::authors::get_authors;
use calibre::models::file_formats::get_file_formats;
use calibre::models::identifier_types::get_identifier_types;
use calibre::models::languages::get_languages;
use calibre::models::publishers::get_publishers;
use calibre::models::tags::get_tags;
use diesel::{PgConnection, SqliteConnection};

use crate::db::get_connection_pool;
use crate::error::{Error, ErrorKind};
use crate::import::db::get_calibre_db;
use crate::import::import_books::import_books;
use crate::models::authors::{add_author, NewAuthor};
use crate::models::file_formats::{add_file_format, NewFileFormat};
use crate::models::identifier_types::{add_identifier_type, NewIdentifierType};
use crate::models::languages::{add_language, NewLanguage};
use crate::models::publishers::{add_publisher, NewPublisher};
use crate::models::tags::{add_tag, NewTag};

fn import_authors(sqlite_conn: &SqliteConnection, pg_conn: &PgConnection) -> Result<(), Error> {
    let limit = 10;
    let mut offset = 0;
    loop {
        let author_list = get_authors(sqlite_conn, limit, offset)?;
        if author_list.is_empty() {
            break;
        }
        offset += author_list.len() as i64;
        log::info!("authors offset: {:?}", offset);

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

fn import_languages(sqlite_conn: &SqliteConnection, pg_conn: &PgConnection) -> Result<(), Error> {
    let lang_list = get_languages(sqlite_conn)?;
    log::info!("lang list len: {}", lang_list.len());
    for lang in lang_list {
        let new_lang = NewLanguage {
            lang_code: lang.lang_code,
        };
        if let Err(err) = add_language(pg_conn, &new_lang) {
            match err.kind() {
                ErrorKind::DbUniqueViolationError => {
                    log::info!("language exists: {:?}", new_lang);
                    continue;
                }
                _ => return Err(err),
            }
        }
    }

    Ok(())
}

fn import_publishers(sqlite_conn: &SqliteConnection, pg_conn: &PgConnection) -> Result<(), Error> {
    let limit = 10;
    let mut offset = 0;
    loop {
        let publisher_list = get_publishers(&sqlite_conn, limit, offset)?;
        if publisher_list.is_empty() {
            break;
        }
        offset += publisher_list.len() as i64;
        log::info!("publisher offset: {}", offset);

        for publisher in publisher_list {
            let new_publisher = NewPublisher {
                sort: publisher.sort.unwrap_or_else(|| publisher.name.clone()),
                name: publisher.name,
            };

            if let Err(err) = add_publisher(pg_conn, &new_publisher) {
                match err.kind() {
                    ErrorKind::DbUniqueViolationError => {
                        log::info!("publisher exists: {:?}", new_publisher);
                        continue;
                    }
                    _ => return Err(err),
                }
            }
        }
    }

    Ok(())
}

fn import_tags(sqlite_conn: &SqliteConnection, pg_conn: &PgConnection) -> Result<(), Error> {
    let limit = 10;
    let mut offset = 0;

    loop {
        let tag_list = get_tags(sqlite_conn, limit, offset)?;
        if tag_list.is_empty() {
            break;
        }
        offset += tag_list.len() as i64;
        log::info!("tags offset: {}", offset);

        for tag in tag_list {
            let new_tag = NewTag { name: tag.name };
            if let Err(err) = add_tag(pg_conn, &new_tag) {
                match err.kind() {
                    ErrorKind::DbUniqueViolationError => {
                        log::info!("tag exists: {:?}", new_tag);
                        continue;
                    }
                    _ => return Err(err),
                }
            }
        }
    }

    Ok(())
}

fn import_file_formats(
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
) -> Result<(), Error> {
    let format_list = get_file_formats(sqlite_conn)?;
    for format in format_list {
        let new_format = NewFileFormat { name: format };
        if let Err(err) = add_file_format(pg_conn, &new_format) {
            match err.kind() {
                ErrorKind::DbUniqueViolationError => {
                    log::info!("file format exists: {:?}", new_format);
                    continue;
                }
                _ => return Err(err),
            }
        }
    }

    Ok(())
}

fn import_identifier_types(
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
) -> Result<(), Error> {
    let identifier_types = get_identifier_types(sqlite_conn)?;
    for identifier_type in identifier_types {
        let new_type = NewIdentifierType {
            name: identifier_type,
        };
        if let Err(err) = add_identifier_type(pg_conn, &new_type) {
            match err.kind() {
                ErrorKind::DbUniqueViolationError => {
                    log::info!("identifier type exists: {:?}", new_type);
                    continue;
                }
                _ => return Err(err),
            }
        }
    }

    Ok(())
}

pub fn new_task(calibre_path: &str) -> Result<(), Error> {
    let calibre_pool = get_calibre_db(calibre_path)?;
    let pg_pool = get_connection_pool()?;
    let sqlite_conn = calibre_pool.get()?;
    let pg_conn = pg_pool.get()?;
    // import_authors(&sqlite_conn, &pg_conn)?;
    // import_languages(&sqlite_conn, &pg_conn)?;
    // import_publishers(&sqlite_conn, &pg_conn)?;
    // import_tags(&sqlite_conn, &pg_conn)?;
    // import_file_formats(&sqlite_conn, &pg_conn)?;
    // import_identifier_types(&sqlite_conn, &pg_conn)?;

    import_books(calibre_path, &sqlite_conn, &pg_conn)?;

    Ok(())
}