// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::PgConnection;
use futures::TryStreamExt;
use mongodb::bson::Document;
use mongodb::options::{ClientOptions, FindOptions};
use mongodb::{Client, Collection};

use backend::db::get_connection_pool;
use backend::error::Error;
use backend::models::categories as models;

async fn query_children(
    collection: &Collection<Document>,
    parent_index: &str,
) -> Result<Vec<Document>, mongodb::error::Error> {
    let filter = mongodb::bson::doc! {"parent_index": parent_index};
    let find_options = FindOptions::builder()
        .sort(mongodb::bson::doc! { "order": 1 })
        .build();
    let mut cursor = collection.find(filter, find_options).await?;
    let mut list = Vec::new();
    while let Some(record) = cursor.try_next().await? {
        list.push(record);
    }

    return Ok(list);
}

fn insert_into_db(pg_conn: &mut PgConnection, documents: &[Document]) -> Result<(), Error> {
    if documents.is_empty() {
        return Ok(());
    }
    let parent_index = documents.first().unwrap().get_str("parent_index")?;
    let parent_id = if parent_index == "0" {
        0
    } else {
        let parent = models::get_category_by_serial_number(pg_conn, parent_index)?;
        parent.id
    };

    for document in documents {
        let new_category = models::NewCategory {
            order_index: document.get_i32("order")?,
            serial_number: document.get_str("index")?,
            name: document.get_str("name")?,
            url: document.get_str("url")?,
            description: None,
            parent: parent_id,
        };
        models::add_category(pg_conn, &new_category)?;
    }

    Ok(())
}

pub fn do_import() -> Result<(), Error> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    let db_name = "spider_man";
    let db = client.database(db_name);
    let collection_name = "clcindex_category_clcindex";
    let collection = db.collection::<Document>(collection_name);
    println!("collection: {:?}", collection);

    let db_pool = get_connection_pool()?;
    let mut pg_conn = db_pool.get()?;
    println!("pg conn is ok");

    let mut todo_list = Vec::new();
    let mut root_list = query_children(&collection, "0").await?;
    println!("root_list: {}", root_list.len());
    insert_into_db(&mut pg_conn, &root_list)?;

    root_list.reverse();
    todo_list.extend(root_list);

    while let Some(next_record) = todo_list.pop() {
        if let Ok(parent_index) = next_record.get_str("index") {
            let mut leaf_list = query_children(&collection, parent_index).await?;
            if !leaf_list.is_empty() {
                println!("leaf of {}, count: {}", parent_index, leaf_list.len());
                insert_into_db(&mut pg_conn, &leaf_list)?;

                leaf_list.reverse();
                todo_list.extend(leaf_list);
            }
        } else {
            // TODO(Shaohua): Throw error
            break;
        }
    }

    Ok(())
}
