// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::http::StatusCode;
use diesel::result::DatabaseErrorKind;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum ErrorKind {
    ConfigError,

    CalibreError,

    DbConnError,
    DbGeneralError,
    DbUniqueViolationError,
    DbForeignKeyViolationError,
    DbNotFoundError,

    IoError,
    ActixBlockingError,
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

    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::from_string(ErrorKind::IoError, format!("{}", err))
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Self {
        Self::from_string(ErrorKind::DbConnError, format!("r2d2 {}", err))
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        match &err {
            diesel::result::Error::DatabaseError(kind, _info) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    Self::from_string(ErrorKind::DbUniqueViolationError, format!("{}", err))
                }
                DatabaseErrorKind::ForeignKeyViolation => {
                    Self::from_string(ErrorKind::DbForeignKeyViolationError, format!("{}", err))
                }
                _ => Self::from_string(ErrorKind::DbGeneralError, format!("{}", err)),
            },
            diesel::result::Error::NotFound => {
                Self::from_string(ErrorKind::DbNotFoundError, format!("{}", err))
            }
            _ => Self::from_string(ErrorKind::DbGeneralError, format!("{}", err)),
        }
    }
}

impl From<actix_web::error::BlockingError> for Error {
    fn from(err: actix_web::error::BlockingError) -> Self {
        Self::from_string(
            ErrorKind::ActixBlockingError,
            format!("Actix blocking error: {:?}", err),
        )
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::from_string(ErrorKind::ConfigError, format!("{:?}", err))
    }
}

impl From<std::ffi::OsString> for Error {
    fn from(err: std::ffi::OsString) -> Self {
        Self::from_string(
            ErrorKind::ConfigError,
            format!("OsString to String err: {:?}", err),
        )
    }
}

impl From<calibre::error::Error> for Error {
    fn from(err: calibre::error::Error) -> Self {
        match err.kind() {
            calibre::error::ErrorKind::DbNotFoundError => {
                Self::from_string(ErrorKind::DbNotFoundError, format!("err: {:?}", err))
            }
            _ => Self::from_string(ErrorKind::CalibreError, format!("err: {:?}", err)),
        }
    }
}

impl actix_web::error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self.kind {
            ErrorKind::ConfigError
            | ErrorKind::CalibreError
            | ErrorKind::DbConnError
            | ErrorKind::DbGeneralError
            | ErrorKind::ActixBlockingError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::DbForeignKeyViolationError
            | ErrorKind::DbUniqueViolationError
            | ErrorKind::IoError => StatusCode::BAD_REQUEST,
            ErrorKind::DbNotFoundError => StatusCode::NOT_FOUND,
        }
    }
}
