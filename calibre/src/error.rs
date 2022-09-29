// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use std::fmt;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    ConfigError,

    DbConnError,
    DbGeneralError,
    DbNotFoundError,

    IoError,
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for Error {}

impl Error {
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

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::from_string(
            ErrorKind::DbGeneralError,
            format!("parse json err: {}", err),
        )
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        match &err {
            diesel::result::Error::NotFound => {
                Self::from_string(ErrorKind::DbNotFoundError, format!("{}", err))
            }
            _ => Self::from_string(ErrorKind::DbGeneralError, format!("{}", err)),
        }
    }
}
