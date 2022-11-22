// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use clap::Command;

use crate::error::Error;

pub const CMD_MIGRATE: &str = "migrate";

pub fn new_cmd() -> Command {
    Command::new(CMD_MIGRATE).about("Migrate database to current version")
}

#[allow(clippy::unnecessary_wraps)]
pub const fn do_migrate() -> Result<(), Error> {
    Ok(())
}
