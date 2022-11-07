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
// TODO(Shaohua): Remove allow-list.
#![allow(
    dead_code,
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::use_self,
    clippy::future_not_send,
    clippy::unused_async,
    clippy::multiple_crate_versions
)]
// Caused by diesel::Insertable trait
#![allow(clippy::extra_unused_lifetimes)]

pub mod cmd;
mod db;
pub mod error;
mod import;
mod migrations;
mod models;
mod routers;
mod schema;
mod settings;
mod views;
