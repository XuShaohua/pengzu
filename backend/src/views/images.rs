// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_files::NamedFile;
use actix_web::web;
use shared::images::ImageQuery;

use crate::db::DbPool;
use crate::error::{Error, ErrorKind};
use crate::settings;

pub async fn get_image_by_path(
    _pool: web::Data<DbPool>,
    query: web::Query<ImageQuery>,
) -> Result<NamedFile, Error> {
    let path = query.path.as_path();
    log::info!("filepath: {:?}", path);
    let valid_extension = match path.extension() {
        Some(extension) => extension == "webp" || extension == "jpg",
        None => false,
    };
    if !valid_extension {
        return Err(Error::from_string(
            ErrorKind::IoError,
            format!("Invalid image format: {:?}", path),
        ));
    }

    let root_dir = settings::get_library_root_dir()?;
    let filepath = root_dir.join(&path);
    if !filepath.starts_with(root_dir) {
        return Err(Error::from_string(
            ErrorKind::IoError,
            format!("Invalid filepath: {:?}", path),
        ));
    }
    let file = NamedFile::open(filepath)?;
    Ok(file.use_last_modified(true))
}
