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
// wasi v0.10 and v0.11
#![allow(clippy::multiple_crate_versions)]

pub mod app;
pub mod components;
pub mod error;
pub mod hooks;
pub mod router;
pub mod services;
pub mod views;
