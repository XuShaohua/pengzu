// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::error::Error;
use crate::import::db::get_calibre_db;

pub fn new_task(calibre_path: &str) -> Result<(), Error> {
    let calibre_pool = get_calibre_db(calibre_path)?;
    let _conn = calibre_pool.get()?;
    println!("calibre pool: ");
    Ok(())
}
