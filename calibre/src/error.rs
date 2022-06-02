// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum ErrorKind {
    ConfigError,

    DbConnError,
    DbGeneralError,
    DbNotFoundError,

    IoError,
}

#[derive(Debug, Clone, Serialize)]
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
}
