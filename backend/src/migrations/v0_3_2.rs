// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

//! Migrate to v0.3.2

use diesel::PgConnection;

use crate::db;
use crate::error::Error;
use crate::models::{authors, books_tags, tags};

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
        // Step 1: Query tag pattern.
        if let Ok(old_tag) = tags::get_tag_by_name_pattern(conn, &name_pattern) {
            let parts: Vec<&str> = old_tag.name.split(pattern).collect();
            let mut new_tag_ids = Vec::with_capacity(parts.len());
            log::info!("parts: {:?}", parts);

            // Step 2: Create new tags.
            for part in parts {
                let new_tag = tags::add_tag(
                    conn,
                    &tags::NewTag {
                        name: part.to_string(),
                    },
                )?;
                new_tag_ids.push(new_tag.id);
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

                    // Step 4: Finally remove old link.
                    books_tags::delete_by_id(conn, old_book_tag.id)?;
                }
            }
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
