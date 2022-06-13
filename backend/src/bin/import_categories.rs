// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use mongodb::bson::Document;
use mongodb::options::ClientOptions;
use mongodb::Client;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("spider_man");
    //let collection_name = "clcindex_category_clcindex";
    let collection_name = "local-test";
    db.create_collection(collection_name, None).await?;
    let collection_handle = db.collection::<Document>(collection_name);
    let _ret = collection_handle.drop(None).await;

    Ok(())
}
