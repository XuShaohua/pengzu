// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum ErrorKind {
    ConfigError,

    DbError,

    IoError,

    ActixError,
}

#[derive(Debug, Clone, Serialize)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for Error {}

impl Error {
    #[must_use]
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_owned(),
        }
    }

    #[must_use]
    pub const fn from_string(kind: ErrorKind, message: String) -> Self {
        Self { kind, message }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::from_string(ErrorKind::IoError, format!("IoError {}", err))
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Self {
        Self::from_string(ErrorKind::DbError, format!("DbError r2d2 {}", err))
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Self::from_string(
            ErrorKind::DbError,
            format!("DbError, diesel result: {:?}", err),
        )
    }
}

impl From<actix_web::error::BlockingError> for Error {
    fn from(err: actix_web::error::BlockingError) -> Self {
        Self::from_string(
            ErrorKind::ActixError,
            format!("Actix blocking error: {:?}", err),
        )
    }
}
