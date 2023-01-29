// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use std::path::Path;

use crate::error::Error;
use crate::parsers::cip::CipRecord;

pub fn parse_cip_from_epub<P: AsRef<Path>>(_path: P) -> Result<CipRecord, Error> {
    todo!()
}
