// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use std::env;
use std::path::PathBuf;

use crate::error::{Error, ErrorKind};

pub fn get_library_root_dir() -> Result<PathBuf, Error> {
    let path = env::var("LIBRARY_ROOT_DIR").map_err(|err| {
        Error::from_string(
            ErrorKind::ConfigError,
            format!("LIBRARY_ROOT_DIR is not set in environment, err: {:?}", err),
        )
    })?;

    Ok(PathBuf::from(path))
}

pub fn get_jwt_secret() -> Result<String, Error> {
    env::var("JWT_SECRET").map_err(|err| {
        Error::from_string(
            ErrorKind::ConfigError,
            format!("JWT_SECRET is not set in environment, err: {:?}", err),
        )
    })
}
