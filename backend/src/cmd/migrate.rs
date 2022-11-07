// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use clap::Command;

use crate::error::Error;
use crate::migrations;

pub const CMD_MIGRATE: &str = "migrate";

pub fn new_cmd() -> Command {
    Command::new(CMD_MIGRATE).about("Migrate database to current version")
}

pub fn do_migrate() -> Result<(), Error> {
    migrations::v0_3_2::migrate()
}
