// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
// TODO(Shaohua): Remove this attribute.
#![allow(dead_code, clippy::module_name_repetitions, clippy::missing_errors_doc)]

#[macro_use]
extern crate diesel;

pub mod error;
pub mod models;
mod schema;
