// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#[macro_use]
extern crate diesel;

mod db;
pub mod error;
mod models;
pub mod routers;
mod schema;
mod views;
