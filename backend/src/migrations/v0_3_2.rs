// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

//! Migrate to v0.3.2

use diesel::PgConnection;

use crate::db;
use crate::error::{Error, ErrorKind};
use crate::models::{authors, books_authors, books_tags, tags};

pub fn migrate() -> Result<(), Error> {
    let db_pool = db::get_connection_pool()?;
    let mut pg_conn = db_pool.get()?;

    split_author_names(&mut pg_conn)?;
    split_tag_names(&mut pg_conn)
}

fn split_tag_names(conn: &mut PgConnection) -> Result<(), Error> {
    // See import_tags() in src/import/new_task.rs
    let patterns = ["&", ";", "；"];
    for pattern in patterns {
        let name_pattern = format!("%{}%", pattern);
        // Step 1: Query tag pattern.
        while let Ok(old_tag) = tags::get_tag_by_name_pattern(conn, &name_pattern) {
            log::info!("Migrate tag: {}, id: {}", old_tag.name, old_tag.id);
            let parts: Vec<&str> = old_tag.name.split(pattern).collect();
            let mut new_tag_ids = Vec::with_capacity(parts.len());

            // Step 2: Create new tags.
            for part in parts {
                let name = part.trim().to_string();
                if name.is_empty() {
                    continue;
                }
                let new_tag = tags::add_tag(conn, &tags::NewTag { name: name.clone() });
                match new_tag {
                    Ok(new_tag) => {
                        new_tag_ids.push(new_tag.id);
                    }
                    Err(err) => match err.kind() {
                        ErrorKind::DbUniqueViolationError => {
                            log::info!("tag exists: {:?}", name);
                            continue;
                        }
                        _ => return Err(err),
                    },
                }
            }

            // If this old_tag is in use, migrate to new tags.
            if let Ok(old_book_tag_list) = books_tags::get_links_by_tag(conn, old_tag.id) {
                for old_book_tag in &old_book_tag_list {
                    // Step 3: Insert new links.
                    for tag_id in &new_tag_ids {
                        books_tags::add_book_tag(
                            conn,
                            &books_tags::NewBookTag {
                                book: old_book_tag.book,
                                tag: *tag_id,
                            },
                        )?;
                    }

                    // Step 4: Remove old link.
                    books_tags::delete_by_id(conn, old_book_tag.id)?;
                }
            }

            // Step 5: Finally remove old tag
            tags::delete_by_id(conn, old_tag.id)?;
        }
    }

    Ok(())
}

fn split_author_names(conn: &mut PgConnection) -> Result<(), Error> {
    // See import_authors() in src/import/new_task.rs
    let patterns = [";", "&", "；", "、"];
    for pattern in patterns {
        let name_pattern = format!("%{}%", pattern);
        // Step 1: Query author pattern.
        while let Ok(old_author) = authors::get_author_by_name_pattern(conn, &name_pattern) {
            log::info!("Migrate author: {}, id: {}", old_author.name, old_author.id);
            let parts: Vec<&str> = old_author.name.split(pattern).collect();
            let mut new_author_ids = Vec::with_capacity(parts.len());

            // Step 2: Create new authors.
            for part in parts {
                let name = part.trim().to_string();
                if name.is_empty() {
                    continue;
                }

                let new_author = authors::add_author(
                    conn,
                    &authors::NewAuthor {
                        name: name.clone(),
                        link: String::new(),
                    },
                );
                match new_author {
                    Ok(new_author) => {
                        new_author_ids.push(new_author.id);
                    }
                    Err(err) => match err.kind() {
                        ErrorKind::DbUniqueViolationError => {
                            log::info!("author exists: {:?}", name);
                            continue;
                        }
                        _ => return Err(err),
                    },
                }
            }

            // If this old_author is in use, migrate to new authors.
            if let Ok(old_book_author_list) =
                books_authors::get_links_by_author(conn, old_author.id)
            {
                for old_book_author in &old_book_author_list {
                    // Step 3: Insert new links.
                    for author_id in &new_author_ids {
                        books_authors::add_book_author(
                            conn,
                            &books_authors::NewBookAuthor {
                                book: old_book_author.book,
                                author: *author_id,
                            },
                        )?;
                    }

                    // Step 4: Remove old link.
                    books_authors::delete_by_id(conn, old_book_author.id)?;
                }
            }

            // Step 5: Finally remove old author
            authors::delete_by_id(conn, old_author.id)?;
        }
    }

    Ok(())
}
