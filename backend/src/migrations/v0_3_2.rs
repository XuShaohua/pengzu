// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

//! Migrate to v0.3.2

use crate::error::Error;

pub fn migrate() -> Result<(), Error> {
    split_author_names()?;
    split_tag_names()
}

fn split_author_names() -> Result<(), Error> {
    todo!()
}

fn split_tag_names() -> Result<(), Error> {
    todo!()
}
