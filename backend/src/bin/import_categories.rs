// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use futures::TryStreamExt;
use mongodb::bson::Document;
use mongodb::options::{ClientOptions, FindOptions};
use mongodb::{Client, Collection};

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

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    let db_name = "spider_man";
    let db = client.database(db_name);
    let collection_name = "clcindex_category_clcindex";
    let collection = db.collection::<Document>(collection_name);

    let mut todo_list = Vec::new();
    let mut root_list = query_children(&collection, "0").await?;
    // TODO(Shaohua): insert into postgresql.
    println!("root_list: {}", root_list.len());
    root_list.reverse();
    todo_list.extend(root_list);
    while let Some(next_record) = todo_list.pop() {
        if let Ok(parent_index) = next_record.get_str("index") {
            let mut leaf_list = query_children(&collection, parent_index).await?;
            if !leaf_list.is_empty() {
                // TODO(Shaohua): insert into postgresql.
                println!("leaf of {}, count: {}", parent_index, leaf_list.len());
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
