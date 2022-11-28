// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_http::error::HttpError;
use actix_web::http::StatusCode;
use diesel::result::DatabaseErrorKind;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ErrorKind {
    ConfigError,
    JwtError,
    InvalidToken,

    CalibreError,

    DbConnError,
    DbGeneralError,
    DbUniqueViolationError,
    DbForeignKeyViolationError,
    DbNotFoundError,

    IoError,
    JsonError,
    RingError,
    AuthFailed,
    ActixBlockingError,
    MongoDbError,
    MongoDbValueAccessError,
    HttpError,
}

unsafe impl Send for ErrorKind {}

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

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Self {
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
        Self::from_string(ErrorKind::ConfigError, format!("{}", err))
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
                Self::from_string(ErrorKind::DbNotFoundError, format!("err: {}", err))
            }
            _ => Self::from_string(ErrorKind::CalibreError, format!("err: {}", err)),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::from_string(ErrorKind::JsonError, format!("{}", err))
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::error::Error> for Error {
    fn from(err: mongodb::error::Error) -> Self {
        Self::from_string(ErrorKind::MongoDbError, format!("{:?}", err))
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::bson::document::ValueAccessError> for Error {
    fn from(err: mongodb::bson::document::ValueAccessError) -> Self {
        Self::from_string(ErrorKind::MongoDbValueAccessError, format!("{:?}", err))
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Self::from_string(ErrorKind::JwtError, format!("{:?}", err))
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Self {
        Self::from_string(ErrorKind::HttpError, format!("{:?}", err))
    }
}

impl From<ring::error::Unspecified> for Error {
    fn from(err: ring::error::Unspecified) -> Self {
        Self::from_string(ErrorKind::RingError, format!("{:?}", err))
    }
}

impl From<data_encoding::DecodeError> for Error {
    fn from(err: data_encoding::DecodeError) -> Self {
        Self::from_string(ErrorKind::RingError, format!("{:?}", err))
    }
}

impl actix_web::error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self.kind {
            ErrorKind::ConfigError
            | ErrorKind::CalibreError
            | ErrorKind::DbConnError
            | ErrorKind::DbGeneralError
            | ErrorKind::JsonError
            | ErrorKind::ActixBlockingError
            | ErrorKind::HttpError
            | ErrorKind::MongoDbError
            | ErrorKind::MongoDbValueAccessError
            | ErrorKind::RingError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::DbForeignKeyViolationError
            | ErrorKind::DbUniqueViolationError
            | ErrorKind::InvalidToken
            | ErrorKind::IoError => StatusCode::BAD_REQUEST,
            ErrorKind::DbNotFoundError => StatusCode::NOT_FOUND,
            ErrorKind::JwtError | ErrorKind::AuthFailed => StatusCode::UNAUTHORIZED,
        }
    }
}
